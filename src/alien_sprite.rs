use sprite::{BA_DIE, SA_ADDSPRITE, SPRITEACTION, BitmapRes, Point, Rect, Sprite, SpriteExt};

use ::{game, rand_int, RES_BLOBBO_BITMAP, RES_BMISSILE_BITMAP, RES_JELLY_BITMAP, RES_JMISSILE_BITMAP, RES_TMISSILE_BITMAP};

//外星人精灵扩展

pub struct AlienSprite{
    
}

impl SpriteExt for AlienSprite{
    fn update(&self, sprite_action:SPRITEACTION) -> SPRITEACTION{
        //检查精灵是否要发射子弹
        match rand_int(0, game().difficulty/2){
            0 => sprite_action | SA_ADDSPRITE,
            _ => sprite_action
        }
    }

    fn add_sprite(&self, sprite:&Sprite)->Sprite{
        
        //创建一个新的子弹精灵
        let bounds = Rect::new(0, 0, 640, 410);
        let pos = sprite.position();
        let mut velocity = Point{x:0, y:0};
        
        let missile_bitmap = 
            match sprite.bitmap().id(){
                RES_BLOBBO_BITMAP =>{
                    velocity.y = 7;
                    BitmapRes::new(RES_BMISSILE_BITMAP, 5, 16)
                }
                RES_JELLY_BITMAP =>{
                    velocity.y = 5;
                    BitmapRes::new(RES_JMISSILE_BITMAP, 9, 16)
                }
                _ =>{
                    velocity.y = 3;
                    BitmapRes::new(RES_TMISSILE_BITMAP, 16, 14)
                }
            };
        let mut sub_sprite = Sprite::with_bounds_action(missile_bitmap, bounds, BA_DIE);
        sub_sprite.set_velocity(velocity.x, velocity.y);
        sub_sprite.set_position(pos.left+sprite.width()/2, pos.bottom);
        sub_sprite
    }
}