//参考: https://www.hellorust.com/setup/wasm-target/
//#![feature(wasm_import_memory)]
//#![wasm_import_memory]
mod sprite;
mod timer;
mod engine;
mod alien_sprite;
mod background;
use std::ptr;
use std::mem::transmute;
use std::ffi::{CString};
use std::os::raw::{c_char};
use engine::{GameEngine, GameEngineHandler};
use background::StarryBackground;
use std::cmp;
use sprite::{Sprite, Point, Rect, BA_BOUNCE, BA_DIE, BA_WRAP, BitmapRes};
use std::io::Read;

//--------------------------------------------
//-------------游戏资源ID----------------------
//--------------------------------------------
pub const RES_SPLASH_BITMAP:i32 = 0;
pub const RES_DESERT_BITMAP:i32 = 1;
pub const RES_CAR_BITMAP:i32 = 2;
pub const RES_SM_CAR_BITMAP:i32 = 3;
pub const RES_MISSILE_BITMAP:i32 = 4;
pub const RES_BLOBBO_BITMAP:i32   = 5;
pub const RES_BMISSILE_BITMAP:i32 = 6;
pub const RES_JELLY_BITMAP:i32    = 7;
pub const RES_JMISSILE_BITMAP:i32 = 8;
pub const RES_TIMMY_BITMAP:i32    = 9;
pub const RES_TMISSILE_BITMAP:i32 = 10;
pub const RES_SM_EXPLOSION_BITMAP:i32 = 11;
pub const RES_LG_EXPLOSION_BITMAP:i32 = 12;
pub const RES_GAME_OVER_BITMAP:i32 = 13;

pub const RES_BMISSILE_SOUND:i32 = 100;
pub const RES_GAMEOVER_SOUND:i32 = 101;
pub const RES_JMISSILE_SOUND:i32 = 102;
pub const RES_LG_EXPLODE_SOUND:i32 = 103;
pub const RES_SM_EXPLODE_SOUND:i32 = 104;
pub const RES_MISSILE_SOUND:i32 = 105;
pub const URL_BACKGROUND_MUSIC:&str = "music.mp3";

//-----------------------------------
//-------------事件ID----------------
pub const EVENT_MOUSE_MOVE:i32 = 0;
pub const EVENT_MOUSE_CLICK:i32 = 1;
pub const EVENT_TOUCH_MOVE:i32 = 10;

//触摸延迟
pub const DRIVE_THRESHOLD:i32 = 3;
pub const CLIENT_WIDTH:i32 = 600;
pub const CLIENT_HEIGHT:i32 = 450;

//SpaceOut游戏主结构体
pub struct SpaceOut{
    engine: GameEngine,
    difficulty:i32,
    game_over:bool,
    demo: bool,
    background:StarryBackground,
    game_over_delay:i32,
    fire_input_delay:i32,
    score:i32,
    num_lives:i32,
    car_sprite_id:f64,
    last_touch: Option<Point>,
    drive_left:i32,
    drive_right:i32,
}

impl SpaceOut{
    fn new()->SpaceOut{
        SpaceOut{
            engine: GameEngine::new(30, CLIENT_WIDTH, CLIENT_HEIGHT, GameHandler{}),
            difficulty: 80,
            game_over: false,
            demo: true,
            background: StarryBackground::default(CLIENT_WIDTH, CLIENT_HEIGHT),
            game_over_delay: 0,
            fire_input_delay: 0,
            score: 0,
            num_lives: 3,
            car_sprite_id: 0.0,
            last_touch: None,
            drive_left: 0,
            drive_right: 0,
        }
    }

    //新游戏
    fn new_game(&mut self){
        //清除所有精灵
        self.engine.clean_up_sprites();
        //初始化游戏变量
        self.fire_input_delay = 0;
        self.score = 0;
        self.num_lives = 3;
        self.difficulty = 80;
        self.game_over = false;

        if self.demo{
            //添加一些外星人
            for _ in 0..6{
                self.add_alien();
            }
        }else{
            //创建汽车
            let mut car_sprite = Sprite::with_bounds_action(
                                BitmapRes::new(RES_CAR_BITMAP, 37, 18),
                                Rect::new(0, 0, CLIENT_WIDTH, CLIENT_HEIGHT), BA_WRAP);
            self.car_sprite_id = car_sprite.id();
            car_sprite.set_position(300, 405);

            self.engine.add_sprite(car_sprite);
            GameEngine::play_music(URL_BACKGROUND_MUSIC);
        }
    }

    //添加外星人
    fn add_alien(&mut self){
        //创建一个随机的外星人精灵
        let bounds = Rect::new(0, 0, CLIENT_WIDTH, 410);
        self.engine.add_sprite(match rand_int(0, 3){
            1 => {
                // Blobbo
                let mut sprite = Sprite::with_bounds_action(
                                BitmapRes::new(RES_BLOBBO_BITMAP, 32, 272),
                                bounds, BA_BOUNCE);
                sprite.set_num_frames(8, false);
                sprite.set_position(match rand_int(0, 2){
                    0=>0,
                    _=>600
                }, rand_int(0, 370));
                sprite.ext(alien_sprite::AlienSprite{});
                sprite.set_velocity(rand_int(0, 7)-2, rand_int(0, 7)-2);
                sprite
            },
            2 => {
                // Jelly
                let mut sprite = Sprite::with_bounds_action(
                                BitmapRes::new(RES_JELLY_BITMAP, 33, 264),
                                bounds, BA_BOUNCE);
                sprite.set_num_frames(8, false);
                sprite.set_position(rand_int(0, CLIENT_WIDTH), rand_int(0, 370));
                sprite.set_velocity(rand_int(0, 5)-2, rand_int(0, 5)+3);
                sprite.ext(alien_sprite::AlienSprite{});
                sprite
            }
            _ =>{
                // Timmy
                let mut sprite = Sprite::with_bounds_action(
                                BitmapRes::new(RES_TIMMY_BITMAP, 33, 136),
                                bounds, BA_WRAP);
                sprite.set_num_frames(8, false);
                sprite.set_position(rand_int(0, CLIENT_WIDTH), rand_int(0, 370));
                sprite.set_velocity(rand_int(0, 7)+3, 0);
                sprite.ext(alien_sprite::AlienSprite{});
                sprite
            }
        });
    }

    //游戏循环
    fn game_cycle(&mut self){
        if !self.game_over {
            if !self.demo {
                // 随机添加外星人
                if rand_int(0, self.difficulty/2) == 0{
                    self.add_alien();
                }
            }
            //更新背景图
            self.background.update();

            //更新精灵
            self.engine.update_sprites();

            //绘制游戏
            self.game_paint();
        }else{
            self.game_over_delay -= 1;
            if self.game_over_delay == 0{
                //停止播放背景音乐，转换到演示模式
                GameEngine::pause_music();
                self.demo = true;
                self.new_game();
            }
        }
    }

    //游戏绘制
    fn game_paint(&self){
        //绘制背景
        self.background.draw();

        //绘制沙漠
        unsafe { draw_image_at(RES_DESERT_BITMAP, 0, 371); }

        //绘制精灵
        self.engine.draw_sprites();

        if self.demo{
            unsafe{
                //绘制闪屏图片
                draw_image_at(RES_SPLASH_BITMAP, 142, 20);

                //绘制控制说明
                fill_style_rgb(255, 255, 255);
                draw_text("点击屏幕->发射导弹".as_bytes(), 220, 300);
                draw_text("       左滑->倒车".as_bytes(), 220, 330);
                draw_text("       右滑->前进".as_bytes(), 220, 360);
            }
        }else{
            //绘制得分
            unsafe {
                fill_style_rgb(255, 255, 255);
                draw_text(format!("得分：{}", self.score).as_str().as_bytes(), 260, 90);
            }

            //绘制剩余生命
            for i in 0..self.num_lives{
                unsafe { draw_image_at(RES_SM_CAR_BITMAP, 520+25*i, 10); }
            }
            if self.game_over{
                unsafe { draw_image_at(RES_GAME_OVER_BITMAP, 170, 100); }
            }
        }
    }

    //碰撞检测
    fn sprite_collision(&mut self, sprite_hitter:&Sprite, sprite_hittee:&Sprite)->bool{
        //检查是否玩家的子弹和外星人相撞
        let hitter = sprite_hitter.bitmap().id();
        let hittee = sprite_hittee.bitmap().id();
        if hitter == RES_MISSILE_BITMAP && (hittee == RES_BLOBBO_BITMAP ||
            hittee == RES_JELLY_BITMAP || hittee == RES_TIMMY_BITMAP) ||
            hittee == RES_MISSILE_BITMAP && (hitter == RES_BLOBBO_BITMAP ||
            hitter == RES_JELLY_BITMAP || hitter == RES_TIMMY_BITMAP){
            //播放小的爆炸声音
            GameEngine::play_sound(RES_SM_EXPLODE_SOUND);
            //杀死子弹和外星人
            self.engine.kill_sprite(sprite_hitter);
            self.engine.kill_sprite(sprite_hittee);

            //在外星人位置创建一个大的爆炸精灵
            let pos:&Rect = if hitter == RES_MISSILE_BITMAP{
                sprite_hittee.position()
            }else{
                sprite_hitter.position()
            };
            let mut sprite = Sprite::from_bitmap(
                BitmapRes::new(RES_LG_EXPLOSION_BITMAP, 33, 272),
                Rect::new(0, 0, CLIENT_WIDTH, CLIENT_HEIGHT));
            sprite.set_num_frames(8, true);
            sprite.set_position(pos.left, pos.top);
            self.engine.add_sprite(sprite);

            //更新得分
            self.score += 25;
            self.difficulty = cmp::max(80-(self.score/20), 20);
        }
        //检查是否有外星人子弹撞到汽车
        if hitter == RES_CAR_BITMAP && (hittee == RES_BMISSILE_BITMAP ||
            hittee == RES_JMISSILE_BITMAP || hittee == RES_TMISSILE_BITMAP) ||
            hittee == RES_CAR_BITMAP && (hitter == RES_BMISSILE_BITMAP ||
            hitter == RES_JMISSILE_BITMAP || hitter == RES_TMISSILE_BITMAP){
            //播放大的爆炸声音
            GameEngine::play_sound(RES_LG_EXPLODE_SOUND);
            //杀死子弹精灵
            if hitter == RES_CAR_BITMAP{
                self.engine.kill_sprite(sprite_hittee);
            }else{
                self.engine.kill_sprite(sprite_hitter);
            }

            //在汽车位置创建一个大的爆炸精灵
            let pos:&Rect = if hitter == RES_CAR_BITMAP{
                sprite_hitter.position()
            }else{
                sprite_hittee.position()
            };
            let mut sprite = Sprite::from_bitmap(
                BitmapRes::new(RES_LG_EXPLOSION_BITMAP, 33, 272),
                Rect::new(0, 0, CLIENT_WIDTH, CLIENT_HEIGHT));
            sprite.set_num_frames(8, true);
            sprite.set_position(pos.left, pos.top);
            self.engine.add_sprite(sprite);

            //移动汽车到起点
            self.engine.get_sprite(self.car_sprite_id).unwrap().set_position(300, 405);
            self.num_lives -= 1;

            //检查游戏是否结束
            if self.num_lives == 0{
                //播放游戏结束声音
                GameEngine::play_sound(RES_GAMEOVER_SOUND);
                self.game_over = true;
                self.game_over_delay = 150;
            }
        }
        false
    }

    //精灵死亡处理
    fn sprite_dying(&mut self, sprite_dying:&Sprite){
        //检查是否子弹精灵死亡
        let bitmap_id = sprite_dying.bitmap().id();
        if bitmap_id == RES_BMISSILE_BITMAP ||
            bitmap_id == RES_JMISSILE_BITMAP ||
            bitmap_id == RES_TMISSILE_BITMAP{
            //播放小的爆炸声音
            if !self.demo{
                GameEngine::play_sound(RES_SM_EXPLODE_SOUND);
            }
            //在子弹位置创建一个小的爆炸精灵
            let mut sprite = Sprite::from_bitmap(
                BitmapRes::new(RES_SM_EXPLOSION_BITMAP, 17, 136),
                Rect::new(0, 0, CLIENT_WIDTH, CLIENT_HEIGHT));
            sprite.set_num_frames(8, true);
            sprite.set_position(sprite_dying.position().left, sprite_dying.position().top);
            self.engine.add_sprite(sprite);
        }
    }

    //点击、触摸事件
    fn on_touch_event(&mut self, event:i32, x:i32, y:i32){
        match event{
            //点击发射炮弹
            EVENT_MOUSE_CLICK => {
                //如果游戏没有开始，启动游戏
                if self.demo || self.game_over{
                    self.demo = false;
                    self.new_game();
                    return;
                }

                let car_left_pos = self.engine.get_sprite(self.car_sprite_id).unwrap().position().left;
                //创建一个新的导弹精灵
                let mut sprite = Sprite::with_bounds_action(
                    BitmapRes::new(RES_MISSILE_BITMAP, 5, 16),
                    Rect::new(0, 0, CLIENT_WIDTH, CLIENT_HEIGHT), BA_DIE);
                sprite.set_position(car_left_pos+15, 400);
                sprite.set_velocity(0, -7);
                self.engine.add_sprite(sprite);

                //播放导弹发射声音
                GameEngine::play_sound(RES_MISSILE_SOUND);
            },

            EVENT_TOUCH_MOVE | EVENT_MOUSE_MOVE => {
                if self.demo{
                    return;
                }
                //移动汽车
                match self.last_touch{
                    Some(touch_point) => {
                        //判断滑动方向
                        if touch_point.x > x{ self.drive_left = cmp::min(self.drive_left+1, DRIVE_THRESHOLD+1);  }//向左
                        if touch_point.x < x{ self.drive_right = cmp::min(self.drive_right+1, DRIVE_THRESHOLD+1); }//向右
                        
                        //判断是否执行
                        let satisfied = self.drive_left>DRIVE_THRESHOLD || self.drive_right>DRIVE_THRESHOLD;

                        let mut car_sprite = self.engine.get_sprite(self.car_sprite_id).unwrap();
                        let (vx, vy) = (car_sprite.velocity().x, car_sprite.velocity().y);
                        if self.drive_left > DRIVE_THRESHOLD {
                            // Move Left
                            car_sprite.set_velocity(cmp::max(vx-3, -6), vy);
                        }else if self.drive_right > DRIVE_THRESHOLD {
                            // Move Right
                            car_sprite.set_velocity(cmp::min(vx+3, 6), vy);
                        }
                        if satisfied {
                            self.drive_left = 0;
                            self.drive_right = 0;
                        }
                    }
                    _ => self.last_touch = Some(Point{x:x, y:y})
                }
            }

            _ => ()
        }
    }

    //加载游戏资源
    fn load_resource(){
        unsafe {
            set_canvas_size(CLIENT_WIDTH, CLIENT_HEIGHT);
            on_window_resize();
            set_text_size("20px Arial".as_bytes());
            add_resource(RES_SPLASH_BITMAP, "Splash1.png".as_ptr());
            add_resource(RES_DESERT_BITMAP, "Desert.png".as_ptr());
            add_resource(RES_CAR_BITMAP, "Car.png".as_ptr());
            add_resource(RES_SM_CAR_BITMAP, "SmCar.png".as_ptr());
            add_resource(RES_MISSILE_BITMAP, "Missile.png".as_ptr());
            add_resource(RES_BLOBBO_BITMAP, "Blobbo.png".as_ptr());
            add_resource(RES_BMISSILE_BITMAP, "BMissile.png".as_ptr());
            add_resource(RES_JELLY_BITMAP, "Jelly.png".as_ptr());
            add_resource(RES_JMISSILE_BITMAP, "JMissile.png".as_ptr());
            add_resource(RES_TIMMY_BITMAP, "Timmy.png".as_ptr());
            add_resource(RES_TMISSILE_BITMAP, "TMissile.png".as_ptr());
            add_resource(RES_SM_EXPLOSION_BITMAP, "SmExplosion.png".as_ptr());
            add_resource(RES_LG_EXPLOSION_BITMAP, "LgExplosion.png".as_ptr());
            add_resource(RES_GAME_OVER_BITMAP, "GameOver.png".as_ptr());

            add_resource(RES_BMISSILE_SOUND, "BMissile.ogg".as_ptr());
            add_resource(RES_GAMEOVER_SOUND, "GameOver.ogg".as_ptr());
            add_resource(RES_JMISSILE_SOUND, "JMissile.ogg".as_ptr());
            add_resource(RES_MISSILE_SOUND, "Missile.ogg".as_ptr());
            add_resource(RES_LG_EXPLODE_SOUND, "LgExplode.ogg".as_ptr()); 
            add_resource(RES_SM_EXPLODE_SOUND, "SmExplode.ogg".as_ptr());
            
            load_resource();
        }
    }

    //显示资源加载进度
    fn on_load_resource_progress(current:i32, total:i32){
        let percent = current as f32 / total as f32;
        let bar_width = 300;
        let bar_height = 26;
        let bar_left = CLIENT_WIDTH/2-bar_width/2;
        let bar_top = CLIENT_HEIGHT/2-bar_height/2;
        unsafe{
            fill_style_rgb(200, 200, 200);
            fill_rect(bar_left, bar_top, bar_width, bar_height);
            fill_style_rgb(120, 120, 255);
            fill_rect(bar_left, bar_top, (bar_width as f32*percent) as i32, bar_height);
        }
    }

    fn on_window_resize(){
        //调整画布大小
        let (window_width, window_height) = unsafe { (get_window_width(), get_window_height()) };
        let (canvas_style_width, canvas_style_height) = 
            if window_width < window_height{
                //竖屏
                (window_width, (window_width as f32/CLIENT_WIDTH as f32 * CLIENT_HEIGHT as f32) as i32)
            }else{
                ((window_height as f32/CLIENT_HEIGHT as f32 * CLIENT_WIDTH as f32) as i32, window_height)
            };
        unsafe {
            set_canvas_style_size(canvas_style_width, canvas_style_height);
            //居中
            set_canvas_margin(
                (window_width-canvas_style_width)/2,
                (window_height-canvas_style_height)/2,
                0,
                0);
        }
    }
}

//游戏引擎回调函数
struct GameHandler{}
impl GameEngineHandler for GameHandler{
    fn sprite_dying(&mut self, sprite_dying:&Sprite){
        game().sprite_dying(sprite_dying);
    }
    fn sprite_collision(&self, sprite_hitter:&Sprite, sprite_hittee:&Sprite)->bool{
        game().sprite_collision(sprite_hitter, sprite_hittee)
    }
    // fn game_cycle(&mut self){
    //     game().game_cycle();
    // }

    // fn game_paint(&self){
    //     game().game_paint();
    // }
}

static mut GAME:*const SpaceOut = ptr::null_mut();

//获取全局的SpaceOut实例
fn game<'a>() -> &'a mut SpaceOut {
    unsafe {
        if GAME.is_null() {
            GAME = transmute(Box::new(SpaceOut::new()));
        }
        transmute(GAME)
    }
}

//导入的JS帮助函数
extern {
    pub fn log();
    pub fn log_int(i:i32);
    pub fn write_buffer(ptr:i32, b0:u8, b1:u8, b2:u8, b3:u8, b4:u8, b5:u8, b6:u8, b7:u8, b8:u8, b9:u8, b10:u8, b11:u8, b12:u8, b13:u8, b14:u8, b15:u8);
    pub fn set_canvas_size(width:i32, height:i32);
    pub fn set_canvas_margin(left:i32, top:i32, right:i32, bottom:i32);
    pub fn set_canvas_style_size(width:i32, height:i32);
    pub fn add_resource(resId:i32, ptr: *const u8);
    pub fn current_time()->f64;
    pub fn random()->f64;
    pub fn load_resource();
    pub fn set_canvas_font();
    pub fn request_animation_frame();
    pub fn get_window_width()->i32;
    pub fn get_window_height()->i32;
    pub fn fill_style_rgb(r:u8, g:u8, b:u8);
    pub fn fill_rect(x:i32, y:i32, width:i32, height:i32);
    pub fn fill_text(x:i32, y:i32);
    pub fn draw_image_at(res_id:i32, x:i32, y:i32);
    pub fn draw_image(res_id:i32, source_x:i32, source_y:i32, source_width:i32, source_height:i32, dest_x:i32, dest_y:i32, dest_width:i32, dest_height:i32);
    pub fn play_sound(res_id:i32);
    pub fn play_music();
    pub fn pause_music();
}
//生成指定范围的随即整数
pub fn rand_int(l:i32, b:i32)->i32{
    unsafe{
        ((random()*(b as f64 - l as f64 + 1.0)).floor()+l as f64) as i32
    }
}

pub fn write_string(s: &[u8]){
    let mut buffer = [0; 16];
    let mut bytes:&[u8] = s;
    let mut ptr = 0;
    while let Ok(bytes_read) = bytes.read(&mut buffer) {
        if bytes_read == 0 { break; }
        if bytes_read<16{
            for i in 0..16-bytes_read{
                buffer[bytes_read+i] = 0;
            }
        }
        unsafe { write_buffer(ptr, buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7],
                        buffer[8], buffer[9], buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]); }
        ptr += 16;
    }
}

pub fn log_string(s: &[u8]){
    write_string(s);
    unsafe{ log(); }
}

pub fn draw_text(s: &[u8], x:i32, y:i32){
    write_string(s);
    unsafe { fill_text(x, y); }
}

//设置canvas渲染字体
// "30px Arial"
pub fn set_text_size(s: &[u8]){
    write_string(s);
    unsafe{ set_canvas_font(); }
}

pub fn log_number(n:i32){
    unsafe{
        log_int(n);
    }
}

//----------------------------------------------
//--------------以下为导出到JS的函数-------------
//----------------------------------------------

#[no_mangle]
pub fn run() {
    //加载所有资源
    SpaceOut::load_resource();
}

#[no_mangle]
pub fn on_window_resize() {
    SpaceOut::on_window_resize();
}

//资源加载进度监听函数
#[no_mangle]
pub fn on_load_resource_progress(current:i32, total:i32){
    SpaceOut::on_load_resource_progress(current, total);
}

//游戏循环主函数(由window.requestAnimationFrame调用)
#[no_mangle]
pub fn draw_frame() {
    let game = game();
    if  game.engine.ready_for_next_frame(){
            game.game_cycle();
            game.game_paint();
    }
    unsafe { request_animation_frame(); }
}

#[no_mangle]
pub fn on_resources_load() {
    //资源加载完成启动游戏
    game().new_game();
    //开始游戏循环
    unsafe { request_animation_frame(); }
}

#[no_mangle]
pub fn on_touch_event(event:i32, x:i32, y:i32){
    //处理鼠标、触摸事件
    game().on_touch_event(event, x, y);
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}