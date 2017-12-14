struct Foo{}
trait TFoo{}
// образец
// ($s:pat) =>   {println!("pat");};

// идентификатор
// ($t:ident) => {println!("ident");};

// мета элемент; то, что находится внутри атрибутов #[...] и #![...]
// ($m:meta) =>  {println!("meta");};

// одиночное дерево токенов
// ($t:tt) =>    {println!("tt");};

macro_rules! explain {

    ($t:ty) =>    {println!("ty");};

    ($b:block) => {println!("block");};
    ($e:expr) =>  {println!("expr");};

    ($i:item) =>  {println!("item");};

}

macro_rules! explain2 {
    ($s:stmt) =>  {println!("stmt");};

}

macro_rules! explain3 {
    ($p:path) =>  {println!("path");};
}

fn main() {

//item
    explain!(struct Foo{});

//expr
    explain!( 10 );

//block
    explain!( {} );

//stmt
    explain2!( let r=5 );
    explain2!( Foo );
    explain2!( 20 );

//type
    explain!(usize);
    explain!(Foo);
    explain!(TFoo);

//path (например, foo, ::std::mem::replace, transmute::<_, int>, …)
    explain3!(::std::mem::replace);

}
