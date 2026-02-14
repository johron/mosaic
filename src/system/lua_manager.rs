use mlua::{Error, Lua};

pub struct LuaManager {
    lua: Lua,
}

impl LuaManager {
    pub fn new() -> Self {
        Self {
            lua: Lua::new()
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        let globals = self.lua.globals();
        //let mos_api_table = L

        globals.set("mos", "test")
    }

    pub fn load_plugin(&self, plugin_id: &str) -> Result<(), Error> {
        let env = self.lua.create_table()?;
        env.set_metatable(Some(self.lua.globals()))?;

        self.lua.load(plugin_id)
            .set_environment(env)
            .exec()
    }
}