use std::io;
extern crate serde;
extern crate rustc_serialize as serl;
use serl::base64::{self, ToBase64};
use serl::hex::FromHex;

fn main() {
    println!("Set 1 solutions");

    //challenge1
    let mut input1:String = String::new();
    io::stdin().read_line(&mut input1).expect("enter a hex string");
    let input1 = input1.trim().to_string();
    let output1: String = hextob64( input1);
    println!("b64:{output1}");


    //challenge2
    let str1 ="1c0111001f010100061a024b53535009181c".to_string();
    let str2 = "686974207468652062756c6c277320657965".to_string();
    println!("{}",fixedxor(str1, str2));


    //challenge 3
    let encstr = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
    for i in 0..16{
        let mut i = String::from(format!("{i:b}"));
        loop{
            if i.len() >= 4{
                break;
            }else{
                i.insert_str(0, "0");
            }
        }
        //println!("{i}");
        let i = to_hex(i.as_str()).to_string();
        
        let key:Vec<char> = i.chars().collect();
        //println!("{:?}",key);
        let key = key[0];
        let decstr = singlexor(encstr.clone(), key);
        let decstr = hextob64(decstr);
        println!("{key}: {decstr}")
    }
}

fn hextob64(hexstring:String)->String{
    let mut hexstr: String = hexstring;
    let mut b64str: String = String::new();
    let mut len: usize = hexstr.len(); 
    let mut counter:i32 = (len%6).try_into().unwrap();
    loop{
        if counter == 0i32 {
            break;
        }
        counter = (counter + 1)%6;
        hexstr.insert_str(0, "0");

    }
    len = hexstr.len();
    let mut locptr:usize = 0;
    loop{
        //println!("{len}");
        if locptr == len {
            break;
        }else{
            let strsnip = (&hexstr[locptr..locptr+6]).to_string();
            //println!("{strsnip}");
            let stradd = strsnip.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
            //println!("{stradd}");
            b64str.push_str(&stradd);
        }
        locptr = locptr + 6;
    }
    //println!("{len}");
    return b64str;

}
fn fixedxor(string1:String,string2:String) -> String{
    let mut strng = String::new();
    let mut counter = 0i32 as usize;
    let len = string1.len();
    let string1:Vec<char>= string1.chars().collect();
    let string2:Vec<char>=string2.chars().collect();
    //println!("{:?},{:?}",string1,string2);
    loop{
        if counter == len {
            break;
        }

        let c1 = to_binary(string1[counter]);
        let c2 = to_binary(string2[counter]);
        //println!("{c1},{c2}");
        let val = xor(c1,c2);
        let val = to_hex(val.as_str()).to_string();
        strng.push_str(&val);
        counter = counter +1;
    }
    strng
}
fn to_binary(c: char) -> String {
    let b = match c {
       '0' => "0000",
       '1' => "0001",
       '2' => "0010",
       '3' => "0011",
       '4' => "0100",
       '5' => "0101",
       '6' => "0110",
       '7' => "0111",
       '8' => "1000",
       '9' => "1001",
       'a' => "1010",
       'b' => "1011",
       'c' => "1100",
       'd' => "1101",
       'e' => "1110",
       'f' => "1111",
        _  => "",
    };

   b.to_string()
}
fn to_hex(b: &str) -> &str {
    match b {
        "0000" => "0",
        "0001" => "1",
        "0010" => "2",
        "0011" => "3",
        "0100" => "4",
        "0101" => "5",
        "0110" => "6",
        "0111" => "7",
        "1000" => "8",
        "1001" => "9",
        "1010" => "a",
        "1011" => "b",
        "1100" => "c",
        "1101" => "d",
        "1110" => "e",
        "1111" => "f",
        _ => "",
    }
}
fn xor(s1:String,s2:String)->String{
    let s1:Vec<char> = s1.chars().collect();
    let s2:Vec<char> = s2.chars().collect();
    //println!("{:?},{:?}",s1,s2);
    let mut s3:Vec<char> = Vec::new();
    for i in 0..=3{
        if s1[i] == s2[i]{
            s3.push('0');
        }else{
            s3.push('1');
        }
    }
    let s3 = String::from_iter(s3);
    s3
}
fn singlexor(s:String , c:char) -> String{
    let mut sgen: String = String::new();
    for _i in 0..=s.len(){
        sgen.push(c);
    }
    //println!("{sgen}");
    let ret = fixedxor(s,sgen);
    ret
}