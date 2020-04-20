use neat::ga::GA;
use neat::phenotype::RunType;
use neat::phenotype::NeuralNet;

const POP_SIZE:i32 = 150;

#[derive(Debug)]
pub enum Turn{
    Left,
    Right
}

pub struct CarBrain{
    ga: GA,
    generation: i32,
    current_brain: usize,
    frame_count: f64,
    max_score: f64,
    use_best: bool,
    best_brain: Option<NeuralNet>,
}

impl CarBrain{
    pub fn new() -> CarBrain{
        let mut ga = GA::new(POP_SIZE, 3, 2);
        ga.create_phenotypes();
        CarBrain{
            ga,
            generation: 0,
            current_brain: 0,
            frame_count: 0.0,
            max_score: 0.0,
            use_best: false,
            best_brain: None,
        }
    }

    //下一代
    fn epoch(&mut self) -> Option<String>{
        self.ga.epoch();
        self.generation += 1;
        self.current_brain = 0;
        //绘制最好的网络
        let brains: Vec<usize> = self.ga.get_best_phenotypes_from_last_generation();
        if brains.len() > 0 {
            self.best_brain = Some(self.ga.get_phenotype(brains[0]).clone());
            let net_img = self.ga.get_phenotype(brains[0]).draw_net(120, 100, 10);
            Some(net_img)
        }else{
            None
        }
    }

    pub fn current_brain(&self) -> usize{
        self.current_brain
    }

    pub fn current_generation(&self) -> i32{
        self.generation
    }

    pub fn use_best(&mut self, use_best:bool){
        self.use_best = use_best;
    }

    pub fn max_score(&self) -> i32{
        self.max_score as i32
    }

    //汽车被子弹击中以后，进入下一个大脑进行控制
    pub fn car_dying(&mut self, score: i32) -> Option<String>{
        //设置网络得分
        self.ga.fitness_scores()[self.current_brain] = self.frame_count+score as f64*10.;
        if self.frame_count>self.max_score{
            self.max_score = self.frame_count;
        }
        self.current_brain += 1;
        self.frame_count = 0.0;//重置网络得分
        //所有网络都已死亡，进入下一代
        if self.current_brain == self.ga.pop_size() as usize{
            self.epoch()
        }else{
            None
        }
    }

    //更新网络得到输出
    //输入：车子位置(x/width), 最近的外星人位置(x/width, y/height)，最近的子弹位置(x/width, y/height)
    pub fn update(&mut self, car_pos: f64, alien_x: f64, alien_y: f64, missile_x: f64, missile_y: f64) -> (Turn, bool){
        self.frame_count += 1.0;
        //网络处理
        let output = if self.use_best && self.best_brain.is_some(){
            self.best_brain.as_mut().unwrap().update(&[car_pos, missile_x, missile_y], RunType::Active)
        }else{
            let phenotype = self.ga.get_phenotype(self.current_brain);
            phenotype.update(&[car_pos, missile_x, missile_y], RunType::Active)
        };
        let fire = output[1] > 0.5;
        if output[0] > 0.5 {
            (Turn::Left, fire)
        }else{
            (Turn::Right, fire)
        }
    }
}