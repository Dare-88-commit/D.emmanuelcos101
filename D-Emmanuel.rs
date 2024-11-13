use std::io;

fn main() {
    for _ in 1..=100{
        
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();

    println!("Enter name");
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let name = input1.trim().to_uppercase();

    println!("Enter date of birth");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let dob = input2.trim();
    println!("Enter email address");
    io::stdin().read_line(&mut input3).expect("not a valid string");
    let email = input3.trim().to_lowercase();

    println!("Enter phone number");
    io::stdin().read_line(&mut input4).expect("not a valid string");
    let number:i32 = input4.trim().parse().expect("not an integer");

    println!("Enter number of siblings");
    io::stdin().read_line(&mut input5).expect("not a valid string");
    let siblings:i32 = input5.trim().parse().expect("not an integer");

    println!("Enter number of children");
    io::stdin().read_line(&mut input6).expect("not a valid string");
    let children:i32 = input6.trim().parse().expect("not an integer");

    println!("Enter medical diagnosis");
    io::stdin().read_line(&mut input7).expect("not a valid string");
    let diagnosis = input7.trim().to_lowercase();

    println!("Enter vilage of residence");
    io::stdin().read_line(&mut input8).expect("not a valid string");
    let residence = input8.trim().to_lowercase();

    println!("Enter age");
    io::stdin().read_line(&mut input9).expect("not a valid string");
    let age:i32 = input9.trim().parse().expect("not an integer");
    println!("Name :{}", name);
    println!("Date of birth :{}", dob);
    println!("Email address :{}", email);
    println!("Phone number :{}", number);
    println!("Number of siblings :{}", siblings);
    println!("Number of children :{}", children);
    println!("Medical diagnosis :{}", diagnosis);
    println!("Village of residence :{}", residence);
        if diagnosis == "alzheimer"{
            if age>50{
                if children>4{
                    if residence== "akigboon"{
                        let amt:i32 = 1_200_000;
                        let discount:i32 = 20;
                        let ca = amt - (amt * (discount / 100));
                        println!("Amount is {}", ca);
                    }else{
                        println!("Amount is 1_200_000");
                    }
                }else{
                    
                    println!("Amount is 1_200_000");
                }
            }else{
                
                println!("Amount is 1_200_000");
            }
        }else{
           
            println!("Amount is 1_200_000");
        };

        if diagnosis == "arthritis"{
            if age==30{
                if siblings>3{
                    if residence== "ngbagi"{
                        let amt:i32 = 550_000;
                        let discount:i32 = 15;
                        let ca:i32 = amt - (amt * (discount / 100));
                        
                        println!("Amount is {}", ca);
                    }else{
                        
                        println!("Amount is 550_000");
                    }
                }else{
                   
                    println!("Amount is 550_000");
                }
            }else{
                        
                println!("Amount is 550_000");
            }
        }else{
                       
            println!("Amount is 550_000");
        };


        if diagnosis == "chronic kidney disease(ckd)"{
            if age>40{
                if children>3{
                    if residence== "akbarianig"{
                        let amt:i32 = 1_500_000;
                        let discount:i32 = 15;
                        let ca:i32 = amt - (amt* (discount / 100));
                        
                        println!("Amount is {}", ca);
                    }else{
                        
                        println!("Amount is 1_500_000");
                    }
                }else{
                    
                    println!("Amount is 1_500_000");
                }
            }else{
                
                println!("Amount is 1_500_000");
            }
        }else{
            
            println!("Amount is 1_500_000");
        };

        if diagnosis == "diabetes"{
            if age>28 && age<45{
                if children>=2 && children<=4{
                    if residence== "oberelemion"{
                        let amt:i32 = 850_000;
                        let discount:i32 = 10;
                        let ca:i32 = amt - (amt * (discount / 100));
                        
                        println!("Amount is {}", ca);
                    }else{
                        
                        println!("Amount is 850_000");
                    }
                }else{
                   
                    println!("Amount is 850_000");
                }
            }else{
               
                println!("Amount is 850_000");
            }
        }else{
            
            println!("Amount is 850_000");
        };

        if diagnosis == "arthritis"{
            if age>58{
                if siblings>5{
                    if children>5{
                        if residence== "nolotowueyie"{
                            let amt:i32 = 480_000;
                            let discount:i32 = 10;
                            let ca:i32 = amt - (amt* (discount / 100));
                       
                            println!("Amount is {}", ca);    
                        }else{
                            println!("Amount is 480_000");
                        }    
                    }else{
                       
                        println!("Amount is 480_000");
                    }
                }else{
                    println!("Amount is 480_000");
                }
            }else{
                println!("Amount is 480_000");
            }
        }else{
            println!("Amount is 480_000");
        };









    }
}