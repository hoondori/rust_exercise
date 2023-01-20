use rand::Rng;

// 전체 미로 크기
const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();

    // 미로 초기화 
    let mut maze = [[0; MAP_N]; MAP_N];

    // 둘레를 벽으로 감싸기
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N-1] = 1;
        maze[MAP_N-1][n] = 1;
    }

    // 2칸마다 1개의 벽을 설치
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 { continue;}
            maze[y][x] = 1; // 벽 설치

            // 상하좌우 중 하나를 랜덤하게 벽으로 만들기
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1,
                1 => maze[y+1][x] = 1,
                2 => maze[y][x-1] = 1,
                3 => maze[y][x+1] = 1,
                _ => {},
            }
        }
    }
    // 미로 출력
    let tiles = ["⬜️", "⬛️"]; // 0과 1을 각각 흰색과 검은색 타일로 치환
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
