use std::fmt;

fn main() {
  println!("{} days", 365); //{} automatically replaced with args stringified

  println!("{0}, this is {1}. {1}, this is {0}", "Jon", "Sarah"); //positional arguments

  println!("{subject} {verb} {object}", object="the silly Tobimaru", subject="the quick brown fox", verb="gets played with by");

  println!("Base 10:      {}", 69420);
  println!("Base 2 (binary) {:b}", 69420);
  println!("Base 8 (octal) {:o}", 69420);
  println!("Base 16 (hex) {:x}", 69420);

  println!("{number:>5}", number=1); //right justify by specified width
  println!("{number:0>5}", number=1); //pad left
  println!("{number:0<5}", number=1); //pad right
  println!("{number:0>width$}", number=1, width=5); //append $ to specify arguments for formatting

  struct Structure{ints: Vec<i32>}
  impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.ints {
            writeln!(f, "{}", i)?;
        }
        Ok(())
    }
}

let mut vec = Vec::new();
vec.push(1);
vec.push(2);
vec.push(3);
println!("{}", Structure{ints: vec});


}