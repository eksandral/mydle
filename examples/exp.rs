use my_idle::components::Experience;

pub fn main() -> anyhow::Result<()> {
    let data: Vec<usize> = vec![1, 5, 68, 363, 1168, 2884, 6038, 11287, 19423, 31378, 48229];
    for (lvl, xp) in data.iter().enumerate() {
        println!("LVL {}", lvl + 2);
        for i in 5..10 {
            println!("{}.ilog({}) = {} ", xp, i, xp.ilog(i));
        }
    }
    Ok(())
}
