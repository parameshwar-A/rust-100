fn main() {

    enum IpKind{
        V4(u8,u8,u8,u8),
        V6(String)
    }


    struct Connection{
        ip_kind: IpKind,
        connection_name: String
}

    let con1 = Connection{
        ip_kind: IpKind::V4(1,1,1,1),
        connection_name: String::from("home")
    };

}
