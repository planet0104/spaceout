use mengine::*;
mod alien_sprite;
mod background;
use background::StarryBackground;
use engine::GameEngine;
use engine::{Resource, Sprite, BA_BOUNCE, BA_DIE, BA_WRAP};
use std::cell::RefCell;
use std::cmp;
use std::collections::HashMap;
use std::rc::Rc;

pub const ASSETS_SPLASH_BITMAP: &str = "Splash.png";
pub const ASSETS_DESERT_BITMAP: &str = "Desert.png";
pub const ASSETS_CAR_BITMAP: &str = "Car.png";
pub const ASSETS_SM_CAR_BITMAP: &str = "SmCar.png";
pub const ASSETS_MISSILE_BITMAP: &str = "Missile.png";
pub const ASSETS_BLOBBO_BITMAP: &str = "Blobbo.png";
pub const ASSETS_BMISSILE_BITMAP: &str = "BMissile.png";
pub const ASSETS_JELLY_BITMAP: &str = "Jelly.png";
pub const ASSETS_JMISSILE_BITMAP: &str = "JMissile.png";
pub const ASSETS_TIMMY_BITMAP: &str = "Timmy.png";
pub const ASSETS_TMISSILE_BITMAP: &str = "TMissile.png";
pub const ASSETS_SM_EXPLOSION_BITMAP: &str = "SmExplosion.png";
pub const ASSETS_LG_EXPLOSION_BITMAP: &str = "LgExplosion.png";
pub const ASSETS_GAME_OVER_BITMAP: &str = "GameOver.png";

pub const ASSETS_BMISSILE_SOUND: &str = "BMissile.ogg";
pub const ASSETS_GAMEOVER_SOUND: &str = "GameOver.ogg";
pub const ASSETS_JMISSILE_SOUND: &str = "JMissile.ogg";
pub const ASSETS_LG_EXPLODE_SOUND: &str = "LgExplode.ogg";
pub const ASSETS_SM_EXPLODE_SOUND: &str = "SmExplode.ogg";
pub const ASSETS_MISSILE_SOUND: &str = "Missile.ogg";
pub const PATH_BACKGROUND_MUSIC: &str = "Music.mp3";

const RESOURCES: &'static [(&'static str, AssetsType); 20] = &[
    (ASSETS_SPLASH_BITMAP, AssetsType::Image),
    (ASSETS_DESERT_BITMAP, AssetsType::Image),
    (ASSETS_CAR_BITMAP, AssetsType::Image),
    (ASSETS_SM_CAR_BITMAP, AssetsType::Image),
    (ASSETS_MISSILE_BITMAP, AssetsType::Image),
    (ASSETS_BLOBBO_BITMAP, AssetsType::Image),
    (ASSETS_BMISSILE_BITMAP, AssetsType::Image),
    (ASSETS_JELLY_BITMAP, AssetsType::Image),
    (ASSETS_JMISSILE_BITMAP, AssetsType::Image),
    (ASSETS_TIMMY_BITMAP, AssetsType::Image),
    (ASSETS_TMISSILE_BITMAP, AssetsType::Image),
    (ASSETS_SM_EXPLOSION_BITMAP, AssetsType::Image),
    (ASSETS_LG_EXPLOSION_BITMAP, AssetsType::Image),
    (ASSETS_GAME_OVER_BITMAP, AssetsType::Image),
    (ASSETS_BMISSILE_SOUND, AssetsType::Sound),
    (ASSETS_GAMEOVER_SOUND, AssetsType::Sound),
    (ASSETS_JMISSILE_SOUND, AssetsType::Sound),
    (ASSETS_LG_EXPLODE_SOUND, AssetsType::Sound),
    (ASSETS_SM_EXPLODE_SOUND, AssetsType::Sound),
    (ASSETS_MISSILE_SOUND, AssetsType::Sound),
];

///游戏资源
pub struct Stage {
    img_splash: Image,
    img_desert: Image,
    img_car: Image,
    img_sm_car: Image,
    img_missile: Image,
    img_blobbo: Image,
    img_bmissile: Image,
    img_jelly: Image,
    img_jmissile: Image,
    img_timmy: Image,
    img_tmissile: Image,
    img_sm_explosion: Image,
    img_lg_explosion: Image,
    img_game_over: Image,

    _sound_bmissile: Sound,
    sound_gameover: Sound,
    _sound_jmissile: Sound,
    sound_lg_explode: Sound,
    sound_sm_explode: Sound,
    sound_missile: Sound,
}

//触摸延迟
pub const DRIVE_THRESHOLD: i32 = 3;
pub const CLIENT_WIDTH: f64 = 600.0;
pub const CLIENT_HEIGHT: f64 = 450.0;

//SpaceOut游戏主结构体
pub struct SpaceOut {
    background: StarryBackground,
    fire_input_delay: i32,
    _last_touch: Option<Point>,
    _drive_left: i32,
    _drive_right: i32,
    sprites: Vec<Sprite>,
    car_sprite_id: f64,
    num_lives: i32,
    score: i32,
    demo: bool,
    difficulty: Rc<RefCell<i32>>,
    resources: HashMap<String, Assets>,
    stage: Option<Stage>,
    game_over: bool,
    game_over_delay: i32,
}

impl SpaceOut {
    //新游戏
    fn new_game(&mut self) {
        //清除所有精灵
        self.clean_up_sprites();
        //初始化游戏变量
        self.fire_input_delay = 0;
        self.score = 0;
        self.num_lives = 3;
        self.difficulty = Rc::new(RefCell::new(80));
        self.game_over = false;
        let stage = self.stage.as_mut().unwrap();
        if self.demo {
            //添加一些外星人
            for _ in 0..6 {
                self.add_alien();
            }
        } else {
            //创建汽车
            let mut car_sprite = Sprite::with_bounds_action(
                String::from("car"),
                Resource::Static(stage.img_car.clone()),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
                BA_WRAP,
            );
            self.car_sprite_id = car_sprite.id();
            car_sprite.set_position(300.0, 405.0);

            self.add_sprite(car_sprite);
            play_music(PATH_BACKGROUND_MUSIC, true);
        }
    }

    //添加外星人
    fn add_alien(&mut self) {
        //创建一个随机的外星人精灵
        let bounds = Rect::new(0.0, 0.0, CLIENT_WIDTH, 410.0);
        let ext = {
            let stage = self.stage.as_ref().unwrap();
            alien_sprite::AlienSprite {
                difficulty: self.difficulty.clone(),
                img_jmissile: stage.img_jmissile.clone(),
                img_tmissile: stage.img_tmissile.clone(),
                img_bmissile: stage.img_bmissile.clone(),
            }
        };
        self.add_sprite(match rand_int(0, 4) {
            1 => {
                // Blobbo
                let mut frames = vec![];
                for y in (0..272).step_by(34) {
                    frames.push([0., y as f64, 32., 34.]);
                }
                let mut anim = Animation::active(
                    self.stage.as_ref().unwrap().img_blobbo.clone(),
                    frames,
                    25.0,
                );
                anim.set_repeat(true);

                let mut sprite = Sprite::with_bounds_action(
                    String::from("blobbo"),
                    Resource::Animation(anim),
                    bounds,
                    BA_BOUNCE,
                );
                sprite.set_position(
                    match rand_int(0, 2) {
                        0 => 0.0,
                        _ => 600.0,
                    },
                    rand_int(0, 370) as f64,
                );
                sprite.ext(ext);
                sprite.set_velocity(rand_int(0, 7) as f64 - 2.0, rand_int(0, 7) as f64 - 2.0);
                sprite
            }
            2 => {
                // Jelly
                let mut frames = vec![];
                for y in (0..264).step_by(33) {
                    frames.push([0., y as f64, 33., 33.]);
                }
                let mut anim =
                    Animation::active(self.stage.as_ref().unwrap().img_jelly.clone(), frames, 25.0);
                anim.set_repeat(true);

                let mut sprite = Sprite::with_bounds_action(
                    String::from("jelly"),
                    Resource::Animation(anim),
                    bounds,
                    BA_BOUNCE,
                );
                sprite.set_position(
                    rand_int(0, CLIENT_WIDTH as i32) as f64,
                    rand_int(0, 370) as f64,
                );
                sprite.set_velocity(rand_int(0, 5) as f64 - 2., rand_int(0, 5) as f64 + 3.);
                sprite.ext(ext);
                sprite
            }
            _ => {
                // Timmy
                let mut frames = vec![];
                for y in (0..136).step_by(17) {
                    frames.push([0., y as f64, 33., 17.]);
                }

                let mut anim =
                    Animation::active(self.stage.as_ref().unwrap().img_timmy.clone(), frames, 25.0);
                anim.set_repeat(true);

                let mut sprite = Sprite::with_bounds_action(
                    String::from("timmy"),
                    Resource::Animation(anim),
                    bounds,
                    BA_WRAP,
                );
                sprite.set_position(
                    rand_int(0, CLIENT_WIDTH as i32) as f64,
                    rand_int(0, 370) as f64,
                );
                sprite.set_velocity(rand_int(0, 7) as f64 + 3., 0.);
                sprite.ext(ext);
                sprite
            }
        });
    }
}

impl State for SpaceOut {
    fn new(window: &mut Window) -> Self {
        window.load_assets(RESOURCES.to_vec());
        SpaceOut {
            background: StarryBackground::default(CLIENT_WIDTH, CLIENT_HEIGHT),
            fire_input_delay: 0,
            _last_touch: None,
            _drive_left: 0,
            _drive_right: 0,
            sprites: vec![],
            car_sprite_id: 0.0,
            num_lives: 3,
            score: 0,
            demo: true,
            resources: HashMap::new(),
            stage: None,
            difficulty: Rc::new(RefCell::new(80)),
            game_over: false,
            game_over_delay: 0,
        }
    }

    fn on_assets_load(
        &mut self,
        path: &str,
        _: AssetsType,
        assets: std::io::Result<Assets>,
        _window: &mut Window,
    ) {
        match assets {
            Ok(assets) => {
                self.resources.insert(path.to_string(), assets);

                if self.resources.len() == RESOURCES.len() {
                    self.stage = Some(Stage {
                        img_splash: self
                            .resources
                            .get(ASSETS_SPLASH_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_desert: self
                            .resources
                            .get(ASSETS_DESERT_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_car: self
                            .resources
                            .get(ASSETS_CAR_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_sm_car: self
                            .resources
                            .get(ASSETS_SM_CAR_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_missile: self
                            .resources
                            .get(ASSETS_MISSILE_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_blobbo: self
                            .resources
                            .get(ASSETS_BLOBBO_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_bmissile: self
                            .resources
                            .get(ASSETS_BMISSILE_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_jelly: self
                            .resources
                            .get(ASSETS_JELLY_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_jmissile: self
                            .resources
                            .get(ASSETS_JMISSILE_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_timmy: self
                            .resources
                            .get(ASSETS_TIMMY_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_tmissile: self
                            .resources
                            .get(ASSETS_TMISSILE_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_sm_explosion: self
                            .resources
                            .get(ASSETS_SM_EXPLOSION_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_lg_explosion: self
                            .resources
                            .get(ASSETS_LG_EXPLOSION_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),
                        img_game_over: self
                            .resources
                            .get(ASSETS_GAME_OVER_BITMAP)
                            .unwrap()
                            .as_image()
                            .unwrap(),

                        _sound_bmissile: self
                            .resources
                            .get(ASSETS_BMISSILE_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                        sound_gameover: self
                            .resources
                            .get(ASSETS_GAMEOVER_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                        _sound_jmissile: self
                            .resources
                            .get(ASSETS_JMISSILE_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                        sound_lg_explode: self
                            .resources
                            .get(ASSETS_LG_EXPLODE_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                        sound_sm_explode: self
                            .resources
                            .get(ASSETS_SM_EXPLODE_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                        sound_missile: self
                            .resources
                            .get(ASSETS_MISSILE_SOUND)
                            .unwrap()
                            .as_sound()
                            .unwrap(),
                    });

                    self.new_game();
                }
            }
            Err(err) => alert(
                "温馨提示",
                &format!("资源文件加载失败:{:?} {:?}", path, err).as_str(),
            ),
        }
    }

    fn event(&mut self, event: Event, _window: &mut Window) {
        if self.stage.is_none() {
            return;
        }
        match event {
            Event::KeyUp(key) => {
                if key.to_lowercase() == "enter" {
                    //如果游戏没有开始，启动游戏
                    if self.demo || self.game_over {
                        self.demo = false;
                        self.new_game();
                        return;
                    }
                }
            }
            Event::Click(_x, _y) => {
                //如果游戏没有开始，启动游戏
                if self.demo || self.game_over {
                    if self.game_over_delay == 0 {
                        self.demo = false;
                        self.new_game();
                    }
                    return;
                }

                let car_left_pos = self.get_sprite(self.car_sprite_id).unwrap().position().left;
                //创建一个新的导弹精灵
                let mut sprite = Sprite::with_bounds_action(
                    String::from("missile"),
                    Resource::Static(self.stage.as_ref().unwrap().img_missile.clone()),
                    Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
                    BA_DIE,
                );
                sprite.set_position(car_left_pos + 15.0, 400.0);
                sprite.set_velocity(0.0, -7.0);
                self.add_sprite(sprite);

                //播放导弹发射声音
                mengine::play_sound(&self.stage.as_ref().unwrap().sound_missile);
            }
            Event::MouseMove(x, _y) => {
                if self.demo {
                    return;
                }
                //直接拖动控制
                let car_sprite = self.get_sprite(self.car_sprite_id).unwrap();
                if x >= 0.0 && x + car_sprite.width() <= CLIENT_WIDTH {
                    car_sprite.set_position(x, car_sprite.position().top);
                }

                //移动汽车
                // match self.last_touch{
                //     Some(touch_point) => {
                //         //判断滑动方向
                //         if touch_point.x > x{ self.drive_left = cmp::min(self.drive_left+1, DRIVE_THRESHOLD+1);  }//向左
                //         if touch_point.x < x{ self.drive_right = cmp::min(self.drive_right+1, DRIVE_THRESHOLD+1); }//向右

                //         //判断是否执行
                //         let satisfied = self.drive_left>DRIVE_THRESHOLD || self.drive_right>DRIVE_THRESHOLD;

                //         let mut car_sprite = self.engine.get_sprite(self.car_sprite_id).unwrap();
                //         let (vx, vy) = (car_sprite.velocity().x, car_sprite.velocity().y);
                //         if self.drive_left > DRIVE_THRESHOLD {
                //             // Move Left
                //             car_sprite.set_velocity(cmp::max(vx-3, -6), vy);
                //         }else if self.drive_right > DRIVE_THRESHOLD {
                //             // Move Right
                //             car_sprite.set_velocity(cmp::min(vx+3, 6), vy);
                //         }
                //         if satisfied {
                //             self.drive_left = 0;
                //             self.drive_right = 0;
                //         }
                //     }
                //     _ => self.last_touch = Some(Point{x:x, y:y})
                // }
            }
            _ => (),
        }
    }

    fn draw(&mut self, g: &mut Graphics, _window: &mut Window) {
        if self.stage.is_none() {
            let progress_bar_height = 30.0;
            let progress_bar_width = CLIENT_WIDTH * 0.8;
            let progress_bar_x = (CLIENT_WIDTH - progress_bar_width) / 2.0;
            let progress_bar_y = CLIENT_HEIGHT / 2.0;
            //进度条背景
            g.fill_rect(
                &[127, 127, 127, 255],
                progress_bar_x,
                progress_bar_y,
                progress_bar_width,
                progress_bar_height,
            );
            //进度条前景
            let progress = self.resources.len() as f64 / RESOURCES.len() as f64;
            g.fill_rect(
                &[10, 10, 128, 255],
                progress_bar_x,
                progress_bar_y,
                progress * progress_bar_width,
                progress_bar_height,
            );
            g.draw_text(
                &format!("Loading {}%", (progress * 100.0) as i32),
                progress_bar_x + 20.0,
                progress_bar_y + 2.0,
                &[255, 255, 255, 255],
                16,
            );
            return;
        }

        //绘制背景
        self.background.draw(g);
        //绘制沙漠
        {
            let stage = self.stage.as_ref().unwrap();
            g.draw_image(
                None,
                &stage.img_desert,
                None,
                Some([
                    0.0,
                    371.0,
                    stage.img_desert.width(),
                    stage.img_desert.height(),
                ]),
            );
        }

        //绘制精灵
        self.draw_sprites(g);

        let stage = self.stage.as_ref().unwrap();

        if self.demo {
            //绘制闪屏图片
            g.draw_image(
                None,
                &stage.img_splash,
                None,
                Some([
                    142.0,
                    30.0,
                    stage.img_splash.width(),
                    stage.img_splash.height(),
                ]),
            );

            //绘制控制说明
            g.draw_text(
                "点击屏幕->发射导弹",
                220.0,
                300.0,
                &[255, 255, 255, 255],
                13,
            );
            g.draw_text("左滑->倒车", 260.0, 330.0, &[255, 255, 255, 255], 13);
            g.draw_text("右滑->前进", 260.0, 360.0, &[255, 255, 255, 255], 13);
        } else {
            //绘制得分
            g.draw_text(
                &format!("得分：{}", self.score),
                260.0,
                90.0,
                &[255, 255, 255, 255],
                13,
            );

            //绘制剩余生命
            for i in 0..self.num_lives {
                g.draw_image(
                    None,
                    &stage.img_sm_car,
                    None,
                    Some([
                        520. + 25. * i as f64,
                        10.0,
                        stage.img_sm_car.width(),
                        stage.img_sm_car.height(),
                    ]),
                );
            }
            if self.game_over {
                g.draw_image(
                    None,
                    &stage.img_game_over,
                    None,
                    Some([
                        170.,
                        100.0,
                        stage.img_game_over.width(),
                        stage.img_game_over.height(),
                    ]),
                );
            }
        }
    }

    fn update(&mut self, _window: &mut Window) {
        if self.stage.is_none() {
            return;
        }
        if !self.game_over {
            if !self.demo {
                // 随机添加外星人
                if rand_int(0, *self.difficulty.borrow()) == 0 {
                    self.add_alien();
                }
            }
            //更新背景图
            self.background.update();

            //更新精灵
            self.update_sprites();
        } else {
            self.game_over_delay -= 1;
            if self.game_over_delay == 0 {
                //停止播放背景音乐，转换到演示模式
                stop_music();
                self.demo = true;
                self.new_game();
            }
        }
    }
}

//游戏引擎回调函数
impl GameEngine for SpaceOut {
    fn sprites_mut(&mut self) -> &mut Vec<Sprite> {
        &mut self.sprites
    }
    fn sprites(&mut self) -> &Vec<Sprite> {
        &self.sprites
    }
    //精灵死亡处理
    fn sprite_dying(&mut self, sprite_dying_id: usize) {
        let stage = self.stage.as_ref().unwrap();
        //检查是否子弹精灵死亡
        if self.sprites[sprite_dying_id].name() == "missile"
            || self.sprites[sprite_dying_id].name() == "amissile"
        {
            //播放小的爆炸声音
            if !self.demo {
                mengine::play_sound(&stage.sound_sm_explode);
            }
            //在子弹位置创建一个小的爆炸精灵
            let mut frames = vec![];
            for y in (0..136).step_by(17) {
                frames.push([0., y as f64, 17., 17.]);
            }
            let anim = Animation::active(stage.img_sm_explosion.clone(), frames, 25.0);

            let mut sprite = Sprite::from_bitmap(
                String::from("sm_explosion"),
                Resource::Animation(anim),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
            );
            {
                let dpos = self.sprites[sprite_dying_id].position();
                sprite.set_position(dpos.left, dpos.top);
            }
            self.add_sprite(sprite);
        }
    }

    //碰撞检测
    fn sprite_collision(&mut self, sprite_hitter_id: usize, sprite_hittee_id: usize) -> bool {
        //检查是否玩家的子弹和外星人相撞
        let hitter = self.sprites[sprite_hitter_id].name().to_string();
        let hittee = self.sprites[sprite_hittee_id].name().to_string();
        if hitter == "missile" && (hittee == "blobbo" || hittee == "jelly" || hittee == "timmy")
            || hittee == "missile" && (hitter == "blobbo" || hitter == "jelly" || hitter == "timmy")
        {
            //播放小的爆炸声音
            mengine::play_sound(&self.stage.as_ref().unwrap().sound_sm_explode);
            //杀死子弹和外星人
            self.sprites[sprite_hitter_id].kill();
            self.sprites[sprite_hittee_id].kill();

            //在外星人位置创建一个大的爆炸精灵
            let pos: &Rect = if hitter == "missile" {
                self.sprites[sprite_hittee_id].position()
            } else {
                self.sprites[sprite_hitter_id].position()
            };

            let mut frames = vec![];
            for y in (0..272).step_by(34) {
                frames.push([0., y as f64, 33., 34.]);
            }
            let anim = Animation::active(
                self.stage.as_ref().unwrap().img_lg_explosion.clone(),
                frames,
                25.0,
            );

            let mut sprite = Sprite::from_bitmap(
                String::from("lg_explosion"),
                Resource::Animation(anim),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
            );
            sprite.set_position(pos.left, pos.top);
            self.add_sprite(sprite);

            //更新得分
            self.score += 25;
            *self.difficulty.borrow_mut() = cmp::max(80 - (self.score / 20), 20);
        }
        //检查是否有外星人子弹撞到汽车
        if hitter == "car" && hittee == "amissile" || hittee == "car" && hitter == "amissile" {
            //播放大的爆炸声音
            mengine::play_sound(&self.stage.as_ref().unwrap().sound_lg_explode);
            //杀死子弹精灵
            if hitter == "car" {
                self.sprites[sprite_hittee_id].kill();
            } else {
                self.sprites[sprite_hitter_id].kill();
            }

            //在汽车位置创建一个大的爆炸精灵
            let pos: &Rect = if hitter == "car" {
                self.sprites[sprite_hitter_id].position()
            } else {
                self.sprites[sprite_hittee_id].position()
            };

            let mut frames = vec![];
            for y in (0..272).step_by(34) {
                frames.push([0., y as f64, 33., 34.]);
            }
            let anim = Animation::active(
                self.stage.as_ref().unwrap().img_lg_explosion.clone(),
                frames,
                25.0,
            );

            let mut sprite = Sprite::from_bitmap(
                String::from("lg_explosion"),
                Resource::Animation(anim),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
            );
            sprite.set_position(pos.left, pos.top);
            self.add_sprite(sprite);

            //移动汽车到起点
            self.get_sprite(self.car_sprite_id)
                .unwrap()
                .set_position(30.0, 405.0);
            self.num_lives -= 1;

            //检查游戏是否结束
            if self.num_lives == 0 {
                //播放游戏结束声音
                mengine::play_sound(&self.stage.as_ref().unwrap().sound_gameover);
                self.game_over = true;
                self.game_over_delay = 150;
            }
        }
        false
    }
}

fn main() {
    mengine::run::<SpaceOut>(
        "SpaceOut",
        CLIENT_WIDTH,
        CLIENT_HEIGHT,
        Settings {
            ups: 30,
            auto_scale: true,
            ..Default::default()
        },
    );
}
