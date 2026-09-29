fn main() {
let name = "Aisha Lawal";
let uni:&str = "Pan-Atlantic University";
let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, Lagos";
println!("Name: {}", name);
println!("University: {}, \nAddress: {}",uni,addr);

let department:&'static str = "Comuter Science";
let school: &'static str = "school of science and technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
