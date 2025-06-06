fn main()
{
    let signednum: i32 = 2000;
    let unsignednum: u32 = 20000;

    println!("Lomadeath signed: {}",signednum);
    println!("Lomadeath unsignednum: {}\n",unsignednum);

    let float1: f32= 200.2000;
    let float2: f64= 200.12121;

    println!("flaotinPoint: {}",float1);
    println!("floatingPoint2: {}\n",float2);

    let bool1: bool=true;
    let bool2: bool=false;
    println!("Lomadeath bool1: {}",bool1);
    println!("Lomadeath bool2: {}\n",bool2);

    let loma_char_1: char='A';
    let loma_char_2: char='❤';
    println!("Lomadeath Lomachar1: {}",loma_char_1);
    println!("Lomadeath Lomachar2: {}\n",loma_char_2);

    let mytuple: (i32,f32,char)=(1,1.2222,'a');
    let (x,y,z)=mytuple;
    println!("Lomadeath Mytuple: {} {} {}\n",x,y,z);

    let array: [i32;5]=[1,2,3,4,5];
    let repeated: [i32;4]=[5;4];
    println!("array: {:?}\n",array);
    println!("repeated: {:?}\n",repeated);

    let string_slice: &str = "Hello Rust! ";
    let owned_string: String = String::from("Owned String");
    println!("String Slice: {}",string_slice);
    println!("owned_string: {}\n",owned_string);

    let inferred_int = 51;

    println!("InferredInt: {}/n",inferred_int);
}