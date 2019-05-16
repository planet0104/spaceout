use mengine::*;
mod alien_sprite;
mod background;
use background::StarryBackground;
use engine::GameEngine;
use engine::{Point, Rect, Resource, Sprite, BA_BOUNCE, BA_DIE, BA_WRAP};
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

///游戏资源
pub struct Resources {
    img_splash: Rc<Image>,
    img_desert: Rc<Image>,
    img_car: Rc<Image>,
    img_sm_car: Rc<Image>,
    img_missile: Rc<Image>,
    img_blobbo: Rc<Image>,
    img_bmissile: Rc<Image>,
    img_jelly: Rc<Image>,
    img_jmissile: Rc<Image>,
    img_timmy: Rc<Image>,
    img_tmissile: Rc<Image>,
    img_sm_explosion: Rc<Image>,
    img_lg_explosion: Rc<Image>,
    img_game_over: Rc<Image>,

    sound_bmissile: RefCell<AssetsFile>,
    sound_gameover: RefCell<AssetsFile>,
    sound_jmissile: RefCell<AssetsFile>,
    sound_lg_explode: RefCell<AssetsFile>,
    sound_sm_explode: RefCell<AssetsFile>,
    sound_missile: RefCell<AssetsFile>,
}

//触摸延迟
pub const DRIVE_THRESHOLD: i32 = 3;
pub const CLIENT_WIDTH: f64 = 600.0;
pub const CLIENT_HEIGHT: f64 = 450.0;

//SpaceOut游戏主结构体
pub struct SpaceOut {
    background: StarryBackground,
    fire_input_delay: i32,
    last_touch: Option<Point>,
    drive_left: i32,
    drive_right: i32,
    sprites: Vec<Sprite>,
    car_sprite_id: f64,
    num_lives: i32,
    score: i32,
    demo: bool,
    difficulty: Rc<RefCell<i32>>,
    resources: Rc<Resources>,
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
        if self.demo {
            //添加一些外星人
            for _ in 0..6 {
                self.add_alien();
            }
        } else {
            //创建汽车
            let mut car_sprite = Sprite::with_bounds_action(
                String::from("car"),
                Resource::Static(self.resources.img_car.clone()),
                Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
                BA_WRAP,
            );
            self.car_sprite_id = car_sprite.id();
            car_sprite.set_position(300.0, 405.0);

            self.add_sprite(car_sprite);
            play_music("Music.mp3", true);
        }
    }

    //添加外星人
    fn add_alien(&mut self) {
        //创建一个随机的外星人精灵
        let bounds = Rect::new(0.0, 0.0, CLIENT_WIDTH, 410.0);
        let ext = alien_sprite::AlienSprite {
            difficulty: self.difficulty.clone(),
            resources: self.resources.clone(),
        };
        self.add_sprite(match rand_int(0, 4) {
            1 => {
                // Blobbo
                let mut frames = vec![];
                for y in (0..272).step_by(34) {
                    frames.push([0., y as f64, 32., 34.]);
                }
                let mut anim = Animation::active(self.resources.img_blobbo.clone(), frames, 25.0);
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
                let mut anim = Animation::active(self.resources.img_jelly.clone(), frames, 25.0);
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
                let mut anim = Animation::active(self.resources.img_timmy.clone(), frames, 25.0);
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

    //显示资源加载进度
    // fn on_load_resource_progress(current:i32, total:i32){
    //     let percent = current as f32 / total as f32;
    //     let bar_width = 300;
    //     let bar_height = 26;
    //     let bar_left = CLIENT_WIDTH/2-bar_width/2;
    //     let bar_top = CLIENT_HEIGHT/2-bar_height/2;
    //     unsafe{
    //         fill_style_rgb(200, 200, 200);
    //         fill_rect(bar_left, bar_top, bar_width, bar_height);
    //         fill_style_rgb(120, 120, 255);
    //         fill_rect(bar_left, bar_top, (bar_width as f32*percent) as i32, bar_height);
    //     }
    // }
}

impl State for SpaceOut {
    fn new(image_loader: &mut ImageLoader) -> Self {
        let resources = Rc::new(Resources {
            img_splash: image_loader.load("Splash.png").unwrap(),
            img_desert: image_loader.load("Desert.png").unwrap(),
            img_car: image_loader.load("Car.png").unwrap(),
            img_sm_car: image_loader.load("SmCar.png").unwrap(),
            img_missile: image_loader.load("Missile.png").unwrap(),
            img_blobbo: image_loader.load("Blobbo.png").unwrap(),
            img_bmissile: image_loader.load("BMissile.png").unwrap(),
            img_jelly: image_loader.load("Jelly.png").unwrap(),
            img_jmissile: image_loader.load("JMissile.png").unwrap(),
            img_timmy: image_loader.load("Timmy.png").unwrap(),
            img_tmissile: image_loader.load("TMissile.png").unwrap(),
            img_sm_explosion: image_loader.load("SmExplosion.png").unwrap(),
            img_lg_explosion: image_loader.load("LgExplosion.png").unwrap(),
            img_game_over: image_loader.load("GameOver.png").unwrap(),

            sound_bmissile: {
                let mut assets = AssetsFile::new("BMissile.ogg");
                assets.load();
                RefCell::new(assets)
            },
            sound_gameover: {
                let mut assets = AssetsFile::new("GameOver.ogg");
                assets.load();
                RefCell::new(assets)
            },
            sound_jmissile: {
                let mut assets = AssetsFile::new("JMissile.ogg");
                assets.load();
                RefCell::new(assets)
            },
            sound_lg_explode: {
                let mut assets = AssetsFile::new("LgExplode.ogg");
                assets.load();
                RefCell::new(assets)
            },
            sound_sm_explode: {
                let mut assets = AssetsFile::new("SmExplode.ogg");
                assets.load();
                RefCell::new(assets)
            },
            sound_missile: {
                let mut assets = AssetsFile::new("Missile.ogg");
                assets.load();
                RefCell::new(assets)
            },
        });

        let mut spaceout = SpaceOut {
            background: StarryBackground::default(CLIENT_WIDTH, CLIENT_HEIGHT),
            fire_input_delay: 0,
            last_touch: None,
            drive_left: 0,
            drive_right: 0,
            sprites: vec![],
            car_sprite_id: 0.0,
            num_lives: 3,
            score: 0,
            demo: true,
            resources,
            difficulty: Rc::new(RefCell::new(80)),
            game_over: false,
            game_over_delay: 0,
        };
        spaceout.new_game();
        spaceout
    }

    fn event(&mut self, event: Event) {
        match event {
            Event::KeyPress(key) => {
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
                    Resource::Static(self.resources.img_missile.clone()),
                    Rect::new(0.0, 0.0, CLIENT_WIDTH, CLIENT_HEIGHT),
                    BA_DIE,
                );
                sprite.set_position(car_left_pos + 15.0, 400.0);
                sprite.set_velocity(0.0, -7.0);
                self.add_sprite(sprite);

                //播放导弹发射声音
                mengine::play_sound(
                    &mut *self.resources.sound_missile.borrow_mut(),
                    AudioType::OGG,
                );
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
        }
    }

    fn draw(&mut self, g: &mut Graphics) -> Result<(), String> {
        //绘制背景
        self.background.draw(g);
        //绘制沙漠
        g.draw_image(
            self.resources.img_desert.as_ref(),
            None,
            Some([
                0.0,
                371.0,
                self.resources.img_desert.width(),
                self.resources.img_desert.height(),
            ]),
        )?;

        //绘制精灵
        self.draw_sprites(g)?;

        if self.demo {
            //绘制闪屏图片
            g.draw_image(
                self.resources.img_splash.as_ref(),
                None,
                Some([
                    142.0,
                    30.0,
                    self.resources.img_splash.width(),
                    self.resources.img_splash.height(),
                ]),
            )?;

            //绘制控制说明
            g.draw_text(
                "点击屏幕->发射导弹",
                220.0,
                300.0,
                &[255, 255, 255, 255],
                13,
            )?;
            g.draw_text("左滑->倒车", 260.0, 330.0, &[255, 255, 255, 255], 13)?;
            g.draw_text("右滑->前进", 260.0, 360.0, &[255, 255, 255, 255], 13)?;
        } else {
            //绘制得分
            g.draw_text(
                &format!("得分：{}", self.score),
                260.0,
                90.0,
                &[255, 255, 255, 255],
                13,
            )?;

            //绘制剩余生命
            for i in 0..self.num_lives {
                g.draw_image(
                    self.resources.img_sm_car.as_ref(),
                    None,
                    Some([
                        520. + 25. * i as f64,
                        10.0,
                        self.resources.img_sm_car.width(),
                        self.resources.img_sm_car.height(),
                    ]),
                )?;
            }
            if self.game_over {
                g.draw_image(
                    self.resources.img_game_over.as_ref(),
                    None,
                    Some([
                        170.,
                        100.0,
                        self.resources.img_game_over.width(),
                        self.resources.img_game_over.height(),
                    ]),
                )?;
            }
        }
        Ok(())
    }

    fn update(&mut self) {
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
        //检查是否子弹精灵死亡
        if self.sprites[sprite_dying_id].name() == "missile"
            || self.sprites[sprite_dying_id].name() == "amissile"
        {
            //播放小的爆炸声音
            if !self.demo {
                mengine::play_sound(
                    &mut *self.resources.sound_sm_explode.borrow_mut(),
                    AudioType::OGG,
                );
            }
            //在子弹位置创建一个小的爆炸精灵
            let mut frames = vec![];
            for y in (0..136).step_by(17) {
                frames.push([0., y as f64, 17., 17.]);
            }
            let anim = Animation::active(self.resources.img_sm_explosion.clone(), frames, 25.0);

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
            mengine::play_sound(
                &mut self.resources.sound_sm_explode.borrow_mut(),
                AudioType::OGG,
            );
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
            let anim = Animation::active(self.resources.img_lg_explosion.clone(), frames, 25.0);

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
            mengine::play_sound(
                &mut *self.resources.sound_lg_explode.borrow_mut(),
                AudioType::OGG,
            );
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
            let anim = Animation::active(self.resources.img_lg_explosion.clone(), frames, 25.0);

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
                mengine::play_sound(
                    &mut *self.resources.sound_gameover.borrow_mut(),
                    AudioType::OGG,
                );
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
            font_file: Some("wqy-micro-hei.ttf"),
            ups: 30,
            auto_scale: true,
            ..Default::default()
        },
    );
}
