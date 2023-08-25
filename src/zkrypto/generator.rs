pub struct Generator {
    p: i64,
    q: i64,
    g: i64,
    q: i64
}

pub fn get_default() -> Generator {
    return Generator{
    p=33599304334943, 
    q=1820705773, 
    g=25395732195142, 
    h=12433296605365};// Default params
}