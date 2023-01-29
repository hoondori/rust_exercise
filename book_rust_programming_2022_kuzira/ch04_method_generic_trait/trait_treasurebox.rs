trait TreasureBox {
    // 기본 메서드
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no  // 지정한 키로만 상자가 열림
    }

    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price : i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("보석 상자. {} 골드 입수", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key_no: i32) -> bool {
        return true  // 어떤 키로도 열림
    }

    fn get_key_no(&self) -> i32 {
        9999 // any key
    }

    fn check(&self) {
        println!("함정 상자. {} 데미지", self.damage);
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않음");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {price: 30, key_no: 1};
    let box2 = TrapBox { damage: 3};
    let box3 = JewelryBox {price: 30, key_no: 2};

    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}