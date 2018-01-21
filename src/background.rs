use sprite::{Point};
use std::cmp;
use ::{fill_style_rgb, rand_int, fill_rect};

//星星背景

pub struct StarryBackground{
    width:i32,
    height:i32,
    num_stars:usize,
    twink_delay:i32,
    stars:[Point; 100],
    star_colors: [(u8, u8, u8); 100]
}

impl StarryBackground{

    pub fn new(width: i32, height:i32, num_stars:usize, twink_delay:i32)->StarryBackground{
        let num_stars = cmp::min(num_stars, 100);
        //创建星星
        let mut stars = [Point{x:0, y:0}; 100];
        let mut star_colors = [(255, 255, 255); 100];
        for i in 0..num_stars{
            stars[i].x = rand_int(0, width);
            stars[i].y = rand_int(0, height);
            star_colors[i] = (128, 128, 128);
        }
        StarryBackground{
            width: width,
            height: height,
            num_stars: num_stars,
            twink_delay: twink_delay,
            stars: stars,
            star_colors: star_colors
        }
    }

    pub fn default(width:i32, height:i32)->StarryBackground{
        StarryBackground::new(width, height, 100, 50)
    }

    pub fn update(&mut self){
        //随机改变星星的颜色以使其闪烁
        for i in 0..self.num_stars{
            if rand_int(0, self.twink_delay) == 0{
                let color = rand_int(0, 256) as u8;
                self.star_colors[i] = (color, color, color);
            }
        }
    }

    pub fn draw(&self){
        unsafe{
            //绘制纯黑色背景
            fill_style_rgb(0, 0, 0);
            fill_rect(0, 0, self.width, self.height);
            //绘制星星
            for i in 0..self.num_stars{
                fill_style_rgb(self.star_colors[i].0, self.star_colors[i].1, self.star_colors[i].2);
                fill_rect(self.stars[i].x, self.stars[i].y, 1, 1);
            }
        }
    }
}