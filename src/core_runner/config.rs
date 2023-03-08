use std::io::Write;
use std::io::Read;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;

//创建一个结构体，用于管理多个我的世界服务器配置文件
pub struct Config {
    //一个Vec，用于存储多个服务器配置文件
    pub servers: Vec<ServerConfig>,
}

//为Config实现构造函数
impl Config {
    //创建一个新的Config
    pub fn new() -> Config {
        Config {
            servers: Vec::new(),
        }
    }
    //添加一个服务器配置文件
    pub fn add_server(&mut self, server: ServerConfig) {
        self.servers.push(server);
    }
    //删除一个服务器配置文件
    pub fn del_server(&mut self, name: String) {
        let mut index = 0;
        for i in 0..self.servers.len() {
            if self.servers[i].name == name {
                index = i;
                break;
            }
        }
        self.servers.remove(index);
    }
    //修改一个服务器配置文件
    pub fn edit_server(&mut self, server: ServerConfig) {
        let mut index = 0;
        for i in 0..self.servers.len() {
            if self.servers[i].name == server.name {
                index = i;
                break;
            }
        }
        self.servers[index] = server;
    }
    //获取一个服务器配置文件
    pub fn get_server(&self, name: String) -> ServerConfig {
        let mut server = ServerConfig {
            name: String::new(),
            version: String::new(),
            path: String::new(),
            args: String::new(),
        };
        for i in 0..self.servers.len() {
            if self.servers[i].name == name {
                server = self.servers[i].clone();
                break;
            }
        }
        server
    }
    //获取一个服务器配置的迭代器
    pub fn get_servers(&self) -> std::slice::Iter<ServerConfig> {
        self.servers.iter()
    }
    //获取一个服务器配置的可变迭代器
    pub fn get_servers_mut(&mut self) -> std::slice::IterMut<ServerConfig> {
        self.servers.iter_mut()
    }
    //将Config持久化
    pub fn save(&self) {
        let mut file = std::fs::File::create("config.json").unwrap();
        let json = serde_json::to_string(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
    //从文件中读取Config
    pub fn load() -> Config {
        let mut file = std::fs::File::open("config.json").unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        let config: Config = serde_json::from_str(&json).unwrap();
        config
    }

}

//为Config实现serde::ser::Serialize接口
impl serde::ser::Serialize for Config {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.servers.len()))?;
        for server in &self.servers {
            seq.serialize_element(server)?;
        }
        seq.end()
    }
}

//为Config实现serde::de::Deserialize接口
impl<'de> serde::de::Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct ConfigVisitor;

        impl<'de> serde::de::Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Config")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Config, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut servers = Vec::new();
                while let Some(server) = seq.next_element()? {
                    servers.push(server);
                }
                Ok(Config {
                    servers,
                })
            }
        }

        deserializer.deserialize_seq(ConfigVisitor)
    }
}

pub struct ServerConfig{
    //服务器名称
    pub name: String,
    //服务器版本
    pub version: String,
    //服务器路径
    pub path: String,
    //服务器启动参数
    pub args: String,
}

//为ServerConfig实现clone方法
impl Clone for ServerConfig {
    fn clone(&self) -> Self {
        ServerConfig {
            name: self.name.clone(),
            version: self.version.clone(),
            path: self.path.clone(),
            args: self.args.clone(),
        }
    }
}

//为ServerConfig实现serde::ser::Serialize接口
impl serde::ser::Serialize for ServerConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("ServerConfig", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("version", &self.version)?;
        state.serialize_field("path", &self.path)?;
        state.serialize_field("args", &self.args)?;
        state.end()
    }
}

//为ServerConfig实现serde::de::Deserialize接口
impl<'de> serde::de::Deserialize<'de> for ServerConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct ServerConfigVisitor;

        impl<'de> serde::de::Visitor<'de> for ServerConfigVisitor {
            type Value = ServerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct ServerConfig")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ServerConfig, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut name = String::new();
                let mut version = String::new();
                let mut path = String::new();
                let mut args = String::new();
                while let Some(key) = map.next_key()? {
                    match key {
                        "name" => {
                            name = map.next_value()?;
                        }
                        "version" => {
                            version = map.next_value()?;
                        }
                        "path" => {
                            path = map.next_value()?;
                        }
                        "args" => {
                            args = map.next_value()?;
                        }
                        _ => {}
                    }
                }
                Ok(ServerConfig {
                    name,
                    version,
                    path,
                    args,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "version", "path", "args"];
        deserializer.deserialize_struct("ServerConfig", FIELDS, ServerConfigVisitor)
    }
}