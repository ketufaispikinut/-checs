use std::{collections::HashMap, f32::INFINITY};

use bracket_terminal::prelude::*;
use rand::{rngs::ThreadRng, thread_rng, Rng, SeedableRng};
embedded_resource!(FONT,"../resources/terminal8x81.png");
fn main() {//"./resources/terminal8x81.png"
    link_resource!(FONT,"../resources/terminal8x81.png");

    //println!("Hello, world!");
    let context = BTermBuilder::new() //mut
        .with_tile_dimensions(8*8/4, 8*8/4)
        .with_font("terminal8x81.png", 8, 8)
        .with_title("échecs") //1Jeu d'échec//chess
        .with_resource_path("./resources")
        .with_simple_console(18, 16, "terminal8x81.png") //42//32//6
        .with_dimensions(18, 16) //42, 32//6
        .with_gutter(4 * 4)
        .build()
        .unwrap();
    let mut jeu = Game::new();
    // not black team
    jeu.field.pieces.insert(
        (3, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Reine,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (3 + 1, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Roi, //Reine
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (2, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Fou,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (1, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Chevalier,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (0, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Tour,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (5, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Fou,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (6, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Chevalier,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (7, 7),
        Piece {
            //7
            team: true,
            piece: PieceType::Tour,
            moved: false,
        },
    );
    for i in 0..8 {
        jeu.field.pieces.insert(
            (i, 6),
            Piece {
                //7
                team: true,
                piece: PieceType::Pion, //Pion//Fou
                moved: false,
            },
        );
    }
    // black team
    jeu.field.pieces.insert(
        (3 + 1, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Reine,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (3 + 1 - 1, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Roi, //Reine
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (2, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Fou,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (1, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Chevalier,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (0, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Tour,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (5, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Fou,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (6, 7 - 7),
        Piece {
            //7
            team: !true,
            piece: PieceType::Chevalier,
            moved: false,
        },
    );
    jeu.field.pieces.insert(
        (7, 7 - 7),
        Piece {
            //7
            team: !true,            //x
            piece: PieceType::Tour, //Tour//Fou
            moved: false,
        },
    );
    for i in 0..8 {
        jeu.field.pieces.insert(
            (i, 6 - 5),
            Piece {
                //7
                team: !true,
                piece: PieceType::Pion, //Pion//Fou
                moved: false,
            },
        );
    }
    main_loop(context, jeu).unwrap();
}
impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        //context
        // ctx.set_active_console(0);//id
        //ctx.set(0, 0, RED, BLACK, 'H');
        //ctx.set_active_console(1);//id
        //ctx.set(0, 0, YELLOW, BLACK, '@');
        ctx.cls(); //;
        let mouse_pos = ctx.mouse_point() / 2;
        self.field.render(
            ctx,
            (0, 0),
            if self.selected != (9, 9) {
                self.selected
            } else {
                (mouse_pos.x as u8, mouse_pos.y as u8)
            },
        ); //);//4//(32-16)/2
      //  if ctx.left_click{
        //    self.opponent_move(!false);
        //    self.opponent_move(false);
            //  }
        //return;
        let occupancy = self.field.occupancy(true);
        let mut my_move = None;
        let mut dd = false;
        if ctx.left_click && mouse_pos.x < 8 && mouse_pos.y < 8 && self.selected == (9, 9) {
            let t = (mouse_pos.x as u8, mouse_pos.y as u8);
            if let Some(k) = self.field.pieces.get(&t) {
                if k.team {
                    self.selected = t;
                }
            }
        }
        //)
        else if self.selected != (9, 9) {
            if ctx.left_click && {
                let t = (mouse_pos.x as u8, mouse_pos.y as u8);
                t != self.selected
            } {
                dd = true;
            } //}
            if let Some(k) = self.field.pieces.get(&self.selected) {
                //k
                let moves = k.get_positions_targets(self.selected, &occupancy); //position
                for i2 in moves {
                    let i=i2.target;
                    let is_in =
                        mouse_pos.x * 2 == (i.0 * 2) as i32 && mouse_pos.y * 2 == (i.1 * 2) as i32;
                    'm:for x in 0..2 {
                        for y in 0..2 {
                            ////////+x//+y
                            if is_in {
                                //&&//||
                                ctx.set_bg(i.0 * 2 + x, i.1 * 2 + y, MAGENTA2);
                                if ctx.left_click {
                                    my_move = Some(i2);
                                    break 'm;
                                    //println!("k");
                                }
                                //println!("t");
                            } else {
                                ctx.set_bg(i.0 * 2 + x, i.1 * 2 + y, BLUE);
                            }
                        }
                    }
                }
            }
            if let Some(k) = my_move {//remove
                {//mut//
                    let n = self.field.pieces.get_mut(&self.selected).unwrap(); //if let Some(k)

                    n.moved = true;
                }
                self.selected = (9, 9);
                self.field=self.field.apply_move(k);//actual
                //s//elf.field.pieces.insert(k, n); //if let Some(k)
                self.opponent_move(false);
            }
        }
        if dd {
            self.selected = (9, 9);
        }
        let advantage_white=(self.field.advantage(true)-200.).max(0.);//_
        let advantage_black=(self.field.advantage(false)-200.).max(0.);////l//;
        let total=(advantage_white+advantage_black).max(0.1);
        ctx.fill_region(Rect::with_size(16, 0, 2, 16), ' ', WHITE,(100,100,100));//glyph//fg, bg

        let white_part=(advantage_white/total)*16.;

        let rounded=white_part.ceil() as i32;//0
        ctx.fill_region(Rect::with_size(16, 16-rounded, 2, 16), ' ', (100,100,100),WHITE);
        let percent=((white_part/16.)*100.).round();
        if percent==100.{
            ctx.print(16,0+15,"@!");//*
        }
        else{

            ctx.print(16,15,percent);
        }
        if percent==0.{
            ctx.print(16,0,"*!");//100.-percent//@
        }
        else{
            ctx.print(16,0,100.-percent);
        }

        //println!("{advantage_white}");
       // if advantage_white==advantage_black{

       //}
    }

}

struct Game {
    field: Field,
    selected: (u8, u8),
}
impl Game {
    fn new() -> Self {
        Self {
            field: Field::new(),
            selected: (9, 9),
        }
    }
    fn opponent_move(&mut self,color:bool){
        let moves=self.field.moves_of(color);//false

        if moves.len()==0{
            return;
        }
        let mut k_best=moves[0];
        let mut k_best_cost=-INFINITY;
        for i in moves{
            let field=self.field.apply_move(i);//actual//false//true
            let mut cost=field.advantage(color)-field.advantage(!color);//field.field.if; //color//color
            //cost=k_best_cost;
            for i in field.moves_of(!color){//true
                let f=field.apply_move(i);//actual//false//true
                let cost_add=field.advantage(color)-f.advantage(!color);//f.co//ield
                if cost_add<cost{
                    cost=cost_add;
                }
                //cost-=cost_add;// /10.
            }
            if cost>k_best_cost{
                k_best_cost=cost;
                k_best=i;
            }
        }
        self.field=self.field.apply_move(k_best);
        //let rng_num=rand::random::<usize>()%moves.len();//threa
        //let the_move=moves[rng_num];//j//+
        //self.field=self.field.apply_move(the_move);//actual
    }
}
#[derive(Clone,Copy)]
struct Piece {
    team: bool,
    piece: PieceType,
    moved: bool,
}
impl Piece {
    fn si_worth(&self)->f32{
        match self.piece{
            PieceType::Reine => 9.,//todo!(),
            PieceType::Roi => 2000.,//todo!(),
            PieceType::Pion => 1.,//todo!(),
            PieceType::Fou => 3.,//todo!(),
            PieceType::Chevalier => 3.,//todo!(),
            PieceType::Tour => 3.,//todo!(),
        }
    }
    fn worth(&self,position:(u8,u8),occupancy:&[[PieceCase;8];8],worth:&[[f32;8];8])->f32{//_
        macro_rules! dist {
            ($a:expr,$b:expr) => {
                (($b.1 as f32-$a.1 as f32).abs()).max(($b.0 as f32-$a.0 as f32).abs())
            };
        }
        let mut k=match self.piece{
            PieceType::Reine => 9.,//todo!()
            PieceType::Roi => 200.,//todo!()
            PieceType::Pion => 1.,//todo!()
            PieceType::Fou => 3.,//todo!()
            PieceType::Chevalier => {
                //3.
                let mut c=3.;
                c+=1.;
                c-=dist!((3,3),position)/10.;// as f32
                c
            },//todo!()
            PieceType::Tour => 5.,//todo!()
        };
        for i in self.get_positions_targets(position, occupancy){
            if occupancy[i.target.0 as usize][i.target.1 as usize]==PieceCase::OppositeTeam{
               // k+=1./2.;
               k+=worth[i.target.0 as usize][i.target.1 as usize]/10.;
            }
        }
        k
    }

    fn get_positions_targets(
        &self,
        position: (u8, u8),
        occupancy: &[[PieceCase; 8]; 8],
    ) -> Vec<Move> {//(u8, u8)
        //bool
        let pos_x = position.0 as i32;
        let pos_y = position.1 as i32;
        let mut v = vec![];
        macro_rules! t {
            ($a:expr,$b:expr) => {
                //tt//tt
                {
                    let x = pos_x + $a;
                    let y = pos_y + if !self.team { $b * -1 } else { $b };
                    if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                        if occupancy[x as usize][y as usize] != PieceCase::MyTeam {
                            v.push(
                                Move{source:position,target:(x as u8, y as u8),kill:None});
                            //true
                        } else {
                            //false
                        }
                    } else {
                        //false
                    }
                }
            };
        }
        macro_rules! t_nokill {
            ($a:expr,$b:expr) => {
                //tt//tt
                {
                    let x = pos_x + $a;
                    let y = pos_y + if !self.team { $b * -1 } else { $b };
                    if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                        if occupancy[x as usize][y as usize] == PieceCase::None {//MyTeam//!=
                            v.push(
                                Move{source:position,target:(x as u8, y as u8),kill:None});
                            //true
                        } else {
                            //false
                        }
                    } else {
                        //false
                    }
                }
            };
        }
        macro_rules! t_kill_only {//no
            ($a:expr,$b:expr) => {
                //tt//tt
                {
                    let x = pos_x + $a;
                    let y = pos_y + if !self.team { $b * -1 } else { $b };
                    if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                        if occupancy[x as usize][y as usize] == PieceCase::OppositeTeam {//MyTeam//!=//None
                            v.push(
                                Move{source:position,target:(x as u8, y as u8),kill:None});
                            //true
                        } else {
                            //false
                        }
                    } else {
                        //false
                    }
                }
            };
        }
        macro_rules! t_r {
            ($a:expr,$b:expr) => {
                //tt//tt
                let mut p = ($a, $b);
                let mut has_collided = false;
                loop {
                    let cancel = has_collided; //mut
                    if cancel {
                        // amazing programming
                        break;
                    }
                    {
                        let x = pos_x + p.0; //$a//1//os
                        let y = pos_y + p.1; //if !self.team{$b*-1}else{$b};//os
                        p.0 += $a;
                        p.1 += $b;
                        if x >= 0 && x <= 7 && y >= 0 && y <= 7 {
                            if (x as u8, y as u8) == position {
                                continue;
                            }
                            if occupancy[x as usize][y as usize] != PieceCase::MyTeam {
                                if occupancy[x as usize][y as usize] == PieceCase::OppositeTeam {
                                    has_collided = true;
                                } else {
                                    //
                                    //break;
                                }
                                v.push(Move{source:position,target:(x as u8, y as u8),kill:None});
                                //true
                            } else {
                                break;
                                //false
                            }
                        } else {
                            break;
                            //false
                        }
                    }
                }
            };
        }
        match self.piece {
            PieceType::Reine => { //todo!()
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 1 - 1) {
                            t_r!(i, j);
                        }
                    }
                }
            }
            PieceType::Roi => {
                //todo!()
                for i in -1..=1 {
                    for j in -1..=1 {
                        if !(i == 0 && j == 1 - 1) {
                            t!(i, j);
                        }
                    }
                }
            }
            PieceType::Pion => {
                //todo!()
                t_kill_only!(-1,-1);
                t_kill_only!(1,-1);
                if self.moved {
                    t_nokill!(0, -1);
                } else {
                    t_nokill!(0, -1);
                    t_nokill!(0, -2);
                }
            }
            PieceType::Fou => {
                t_r!(-1, -1);
                t_r!(1, 1);
                t_r!(-1, 1);
                t_r!(1, -1);
            } //todo!()},
            PieceType::Chevalier => {
                t!(-1, 2);
                t!(1, -2);
                t!(-1, -2);
                t!(1, 2);
                t!(2, 1); //,
                t!(-2, 1);
                t!(2, -1);
                t!(-2, -1);
            } //todo!()},
            PieceType::Tour => {
                t_r!(1, 0);
                t_r!(-1, 0);
                t_r!(0, 1);
                t_r!(0, -1);
            } //todo!()},
        }
        v
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum PieceType {
    Reine = '@' as u8,
    Roi = '*' as u8,
    Pion = '!' as u8,
    Fou = '?' as u8,
    Chevalier = '$' as u8,
    Tour = '%' as u8,
}
#[derive(Clone)]//,Copy
struct Field {
    pub(crate) pieces: HashMap<(u8, u8), Piece>,
}
#[derive(Clone,Copy,PartialEq)]
struct Move{
    source:(u8,u8),target:(u8,u8),kill:Option<(u8,u8)>
}
#[derive(Clone, Copy, PartialEq)]
enum PieceCase {
    None,
    OppositeTeam,
    MyTeam, //MySelf
}
impl Field {
    fn moves_of(&self,color:bool)->Vec<Move>{
        let mut n=vec![];
        let occu=self.occupancy(color);//team
        for i in &self.pieces{//position
            if i.1.team==color{
                n.append(&mut i.1.get_positions_targets(*i.0, &occu));//pancy//*
            }
        }
        n//j
    }
    fn worth(&self)->[[f32;8];8]{
        let mut n = [[0.; 8]; 8];//PieceCase::None
        for i in &self.pieces {
            //if i.1.team!=team{

            //}
            n[i.0 .0 as usize][i.0 .1 as usize] = i.1.si_worth();//if i.1.team == team {
                //   PieceCase::MyTeam
                //} else {
            //    PieceCase::OppositeTeam
            //};
        }
        n
    }
    /// name is misleading
    pub fn advantage(&self,color:bool)->f32{
        let mut accumulator=0.;
        let co=self.occupancy(color);//team
        let wo=self.worth();//(co)
        for i in &self.pieces{
            if i.1.team==color{
                accumulator+=i.1.worth(*i.0,&co,&wo);//&
            }
          // else{
           //     accumulator+=-i.1.worth(*i.0);//&
           //   }
        }
        accumulator
    }
    pub fn apply_move(&self, actual:Move)->Field{
        let mut f=self.clone();
        let n=f.pieces.remove(&actual.source).unwrap();
        f.pieces.insert(actual.target,n);
        if let Some(k)=actual.kill{
            f.pieces.remove(&k);
        }
        //f
        f
    }
    fn occupancy(&self, team: bool) -> [[PieceCase; 8]; 8] {
        //bool
        let mut n = [[PieceCase::None; 8]; 8];
        for i in &self.pieces {
            //if i.1.team!=team{

            //}
            n[i.0 .0 as usize][i.0 .1 as usize] = if i.1.team == team {
                PieceCase::MyTeam
            } else {
                PieceCase::OppositeTeam
            };
        }
        n
    }
    fn new() -> Self {
        Self {
            pieces: HashMap::new(),
        }
    }
    fn render(&self, ctx: &mut BTerm, offset: (i32, i32), selected: (u8, u8)) {
        let mut rng = rand::rngs::StdRng::from_seed([0; 32]); //0
        let mut grid = [[(0, 0); 8]; 8];
        for i in &mut grid {
            for i in i {
                i.0 = rng.gen_range(0..=1);
                i.1 = rng.gen_range(0..=1);
            }
        }
        let mut k_is = [[false; 16]; 16];
        for i in &self.pieces {
            let char = i.1.piece as u8 as char;
            let t = grid[i.0 .0 as usize][i.0 .1 as usize];
            let nx = t.0; //(i.0.0 as u8^i.0.1 as u8+i.0.0)%2;//BLACK
            let ny = t.1; //(i.0.0 as u8|i.0.1 as u8+i.0.1*i.0.1)%2;//+//x//DARKGRAY
            ctx.set(
                offset.0 + i.0 .0 as i32 * 2 + nx as i32,
                offset.1 + i.0 .1 as i32 * 2 + ny as i32,
                if i.1.team {
                    WHITE
                } else {
                    (50 * 2, 50 * 2, 50 * 2)
                },
                MAGENTA,
                char,
            );
            //ctx.set//print
            k_is[(offset.0 + i.0 .0 as i32 * 2 + nx as i32) as usize]
                [(offset.1 + i.0 .1 as i32 * 2 + ny as i32) as usize] = true;
            //ctx.
        }
        for x in 0..8 {
            for y in 0..8 {
                let black = (x + y) % 2 == 0;
                if !black {
                    for x in x * 2..x * 2 + 2 {
                        for y in y * 2..y * 2 + 2 {
                            //fg
                            // colors taken from https://colorhunt.co/palette/e7ccccede8dca5b68dc1cfa1
                            let v = rng.gen_range(-10..=10); //0
                            let v2 = rng.gen_range(-30..=-10);
                            ctx.set_bg(
                                offset.0 + x,
                                offset.1 + y,
                                (
                                    0xC1i32.saturating_add(v) as u8,
                                    0xCFi32.saturating_add(v) as u8,
                                    0xA1i32.saturating_add(v) as u8,
                                ),
                            ); //WHITE, //,' '//SANDYBROWN//FA//I rip//0 rip
                            let d=rng.gen_range(0..=5);
                            if !k_is[x as usize][y as usize] && d > 4 - 3 {
                                //2
                                ctx.print_color(x,y,/*(0xA6i32.saturating_add(v2) as u8,0xB6i32.saturating_add(v2) as u8,0x8Di32.saturating_add(v2) as u8),(0xC1i32.saturating_add(v) as u8,0xCFi32.saturating_add(v) as u8,0xA1i32.saturating_add(v) as u8)*/(0xC1i32.saturating_add(v2) as u8,0xCFi32.saturating_add(v2) as u8,0xA1i32.saturating_add(v2) as u8),(0xC1i32.saturating_add(v/*2*/) as u8,0xCFi32.saturating_add(v/*2*/) as u8,0xA1i32.saturating_add(v) as u8),/*(0xA6i32.saturating_add(v) as u8,0xB6i32.saturating_add(v) as u8,0x8Di32.saturating_add(v) as u8))*/'"');
                                //,//,
                            }
                        }
                    }
                } else {
                    for x in x * 2..x * 2 + 2 {
                        for y in y * 2..y * 2 + 2 {
                            let v = rng.gen_range(-10..=10); //0
                            let v2 = rng.gen_range(-30..=-10);
                            ctx.set_bg(
                                offset.0 + x,
                                offset.1 + y,
                                (
                                    0xA6i32.saturating_add(v) as u8,
                                    0xB6i32.saturating_add(v) as u8,
                                    0x8Di32.saturating_add(v) as u8,
                                ),
                            ); //WHITE,//,' '//BROWN3
                            let d=rng.gen_range(0..=5);
                            if !k_is[x as usize][y as usize] && d > 4 - 3 {
                                ctx.print_color(
                                    x,
                                    y,
                                    (
                                        0xA6i32.saturating_add(v2) as u8,
                                        0xB6i32.saturating_add(v2) as u8,
                                        0x8Di32.saturating_add(v2) as u8,
                                    ),
                                    (
                                        0xA6i32.saturating_add(v) as u8,
                                        0xB6i32.saturating_add(v) as u8,
                                        0x8Di32.saturating_add(v) as u8,
                                    ),
                                    '"',
                                ); //,
                            }
                        }
                    }
                }
            }
        }
        let pos = ctx.mouse_point(); //s
        let pos = (pos.x, pos.y); // rip à
        let pos_y = if pos.1 < 4 * 2 { 16 - 1 } else { 0 }; //7*2
        if pos.0 / 2 <= 7 && pos.1 / 2 <= 7 {
            let pos = selected;
            let n = grid[pos.0 as usize][pos.1 as usize]; // /2///2
                                                          //ctx.
                                                          //if ((ctx.frame_time_ms*10.) as i32)%2==0{
            ctx.set_bg(pos.0 * 2 + n.0, pos.1 * 2 + n.1, MAGENTA); //YEL//pos.0/2*2+n.0, pos.1/2*2+n.1LOW///2///2
            if let Some(k) = self.pieces.get(&(pos.0 as u8, pos.1 as u8)) {
                // /2///2
                let string = format!("{:?} des {}", k.piece, if k.team { "B" } else { "N" }); //blancs//noirs

                let w = string.len();
                let offset_x = (16 - w) / 2; //0//-
                ctx.print_color(offset_x, pos_y, WHITE, BLACK, string);
                //ctx.print(0,pos_y,string);//
            }
            //}
        }
    }
}
