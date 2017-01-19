enum WorkInfo
{
    Since(i32),
    Division(&'static str),
    Group(&'static str),
    Occupation(&'static str)
}

struct Smith;

trait Indivisual
{
    fn new() -> Self;
    
    fn name(&self) -> &'static str;
    fn age(&self)  -> i32;
}

trait Employee
{
    fn new() -> Self;

    fn since(&self)      -> WorkInfo;
    fn division(&self)   -> WorkInfo;
    fn group(&self)      -> WorkInfo;
    fn occupation(&self) -> WorkInfo;
}

impl Indivisual for Smith
{
    fn new() -> Self { Smith }
    
    fn name(&self) -> &'static str { "smith" }
    fn age(&self)  ->          i32 { 33      }
}

impl Employee for Smith
{
    fn new() -> Self { Smith }
    
    fn since(&self)      -> WorkInfo { WorkInfo::Since(2011)                  }
    fn division(&self)   -> WorkInfo { WorkInfo::Division("Client Architect") }
    fn group(&self)      -> WorkInfo { WorkInfo::Group("Game Architect")      }
    fn occupation(&self) -> WorkInfo { WorkInfo::Occupation("Engineer")       }
}

impl Smith
{
    fn introduce(&self)
    {
        println!("As an indivisual");
        
        println!("  name       : {}", self.name());
        println!("  age        : {}", self.age());
        
        println!("As an employee");
        
        self.print_workinfo(self.since());
        self.print_workinfo(self.division());
        self.print_workinfo(self.group());
        self.print_workinfo(self.occupation());
    }
    
    fn print_workinfo(&self, info: WorkInfo)
    {
        match info {
            WorkInfo::Since(since)       => println!("  since      : {}", since),
            WorkInfo::Division(div)      => println!("  division   : {}", div),
            WorkInfo::Group(group)       => println!("  group      : {}", group),
            WorkInfo::Occupation(occupy) => println!("  occupation : {}", occupy)
        }
    }
}

fn main()
{
    let me = Smith;
    me.introduce();
}
