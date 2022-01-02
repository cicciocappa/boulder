use crate::Color;
#[derive(Clone)]
pub struct Cave {
    pub amoeba_wall_time: u8,
    pub diamond_point: u8,
    pub extra_point: u8,
    pub diamond_needed: [u8; 5],
    pub cave_time: [u16; 5],
    pub colors: [Color; 4],
    pub layout: &'static str,
}

pub fn get_cave(index: usize) -> Cave {
    let avail = [
        Cave {
            amoeba_wall_time: 20,
            diamond_point: 20,
            extra_point: 50,
            diamond_needed: [2, 12, 9, 13, 10],
            cave_time: [150, 110, 70, 70, 70],
            colors: [
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                Color {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                },
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            ],
            layout: "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
W...... ..d.r .....r.r....... ....r....W
W.rRr...... .........rd..r.... ..... ..W
W.......... ..r.....r.r..r........r....W
Wr.rr   b     r......r..r....r...r.....W
Wr. r......... r..r........r......r.rr.W
W... ..r........r.....r. r........r.rr.W
Wwwwwwwwwwwwwwwwwwwwwwwwwwwwwww...r..r.W
W. ...r..d. ..r.r..........d.rd...... .W
W..d.....r..... ........rr r..r....r...W
W...r..r.r..............r .r..r........W
W.r.....r........rrr.......r.. .d....r.W
W.d.. ..r.  .....r.rd..d....r...r..d. .W
W. r..............r r..r........d.....rW
W........wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwW
W r.........r...d....r.....r...r..ddX..W
W r......... r..r........r......r.rr..PW
W. ..r........r.....r.  ....d...r.rr...W
W....rd..r........r......r.rd......r...W
W... ..r. ..r.rr.........r.rd...... ..rW
W.d.... ..... ......... .r..r....r...r.W
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
        },
        Cave {
            amoeba_wall_time: 20,
            diamond_point: 10,
            extra_point: 15,
            diamond_needed: [2, 12, 12, 12, 12],
            cave_time: [150, 110, 70, 40, 30],
            colors: [
                Color {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                Color {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                },
                Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                },
            ],

            layout: "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW
W.r..r..w.r...d.w... .r.wr......w..rr..W
W.......w......rwrr. ...w ..d...w....r.W
W                                      W
Wd......w.r....rw.r. .. w..r..d.w..r.r.W
W.......w.r....rw.r. r..w.....r.w... ..W
Wwwwwwwwwwwwwwwwwwww wwwwwwwwwwwwwwwwwwW
W....rr.w..r....w... ..rw....r..w.....rW
W.......w.. ....w... ...w....r. w.....rW
W                                      W
Wr..r...w....r..w..r ...w......dwr.....W
Wr....r.w..r..r.w... . rw.......wr...r.W
W.r.....w...r...w... . rw.......w r..r.W
Wwwwwwwwwwwwwwwwwwww wwwwwwwwwwwwwwwwwwW
Wr.  q..w....r.rw... ...w.rd..r.w......W
W.....r.wr......w..d ...w ..r...w.r.rr.W
W                                      W
Wd.. .r.wr....r.w.r. ..rw.r.r...w......W
W.....r.wr..d...w... r..w..r....w...rr W
W.d... rw..r....w.Xd r..w. .....w...rr W
W.r.... w.. ..r.w.P. ...w....r.rw.... .W
WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
        },
    ];
    
    avail[index].clone()
}
