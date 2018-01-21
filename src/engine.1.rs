use sprite::{Sprite, SA_ADDSPRITE, SA_KILL};
use timer::Timer;
use ::{play_sound, log_string, play_music, pause_music};

//GameEngine 负责创建游戏窗口、绘制和更新精灵

pub trait GameEngineHandler{
    fn sprite_dying(&mut self, sprite_dying:&Sprite);
    fn sprite_collision(&self, sprite_hitter:&Sprite, sprite_hittee:&Sprite)->bool;
    fn game_cycle(&mut self);
    fn game_paint(&self);
}

pub struct GameEngine{
    handler:Box<GameEngineHandler>,
    timer:Timer,
    width:i32,
    height:i32,
    sprites:Vec<Sprite>
}

impl GameEngine{
    pub fn new<T: GameEngineHandler + 'static>(fps:u64, width:i32, height:i32, handler:T)->GameEngine{
        GameEngine{
            handler:Box::new(handler),
            timer:Timer::new(fps),
            width: width,
            height: height,
            sprites: vec![]
        }
    }

    pub fn add_sprite(&mut self, sprite:Sprite){
        if self.sprites.len()>0 {
            for i in 0..self.sprites.len(){
                //根据z-order插入精灵到数组
                if sprite.z_order() < self.sprites[i].z_order(){
                    self.sprites.insert(i, sprite);
                    return;
                }
            }
        }
        //精灵的zOrder是最高的，放入Vec的末尾
        self.sprites.push(sprite);
    }

    pub fn draw_sprites(&self){
        //绘制所有的精灵
        for sprite in &self.sprites{
            sprite.draw();
        }
    }

    pub fn update_sprites(&mut self){
        //更新所有精灵
        let mut sprites_to_add:Vec<Sprite> = vec![];
        let mut sprites_to_kill:Vec<usize> = vec![];
        for i in 0..self.sprites.len(){
            //保存旧的精灵位置以防需要恢复
            let old_sprite_pos = *self.sprites[i].position();
            //更新精灵
            let sprite_action = self.sprites[i].update();

            //处理SA_ADDSPRITE
            if sprite_action == SA_ADDSPRITE{
                //允许精灵添加它的精灵
                if let Some(sprite) = self.sprites[i].add_sprite(){
                    sprites_to_add.push(sprite);
                }
            }

            //处理 SA_KILL
            if sprite_action == SA_KILL{
                //通知游戏精灵死亡
                self.handler.sprite_dying(&self.sprites[i]);
                //杀死精灵
                sprites_to_kill.push(i);
                continue;
            }

            if self.check_sprite_collision(i){
                self.sprites[i].set_position_rect(old_sprite_pos);
            }
        }

        for sprite in sprites_to_add{
            self.add_sprite(sprite);
        }
        for sprite_id in sprites_to_kill{
            self.sprites.remove(sprite_id);
        }
    }

    pub fn check_sprite_collision(&mut self, test_sprite_id:usize)->bool{
        //检查精灵是否和其他精灵相撞
        let test_sprite = &self.sprites[test_sprite_id];
        for i in 0..self.sprites.len(){
            //不检查精灵自己 和已经死亡的精灵
            if i == test_sprite_id || self.sprites[i].dying() {
                continue;
            }
            if test_sprite.test_collison(self.sprites[i].position()){
                return self.handler.sprite_collision(&self.sprites[i], test_sprite);
            }
        }
        return false;
    }

    pub fn clean_up_sprites(&mut self){
        self.sprites.clear();
    }

    pub fn is_point_in_sprite(&self, x:i32, y:i32)->Option<&Sprite>{
        for sprite in &self.sprites{
            if !sprite.hidden() && sprite.is_point_inside(x, y){
                return Some(sprite);
            }
        }
        None
    }

    pub fn get_sprite(&mut self, id:usize)->Option<&mut Sprite>{
        for sprite in &mut self.sprites{
            if sprite.id() == id{
                return Some(sprite);
            }
        }
        None
    }

    pub fn initialize(&mut self)->bool{
        true
    }

    pub fn end(&self){

    }

    pub fn play_music(url:&str){
        unsafe{
            play_music(url.as_ptr());
        }
    }

    pub fn pause_music(){
        unsafe{
            pause_music();
        }
    }

    pub fn play_sound(res_id:i32){
        unsafe{
            play_sound(res_id);
        }
    }

    pub fn ready_for_next_frame(&mut self)->bool{
        self.timer.ready_for_next_frame()
    }

    pub fn kill_sprite(&mut self, sprite:&Sprite){
        log_string(format!("kill_sprite id={}", sprite.id()).as_str().as_bytes());
        if let Some(s) = self.get_sprite(sprite.id()){
            s.kill();
        }
    }
}