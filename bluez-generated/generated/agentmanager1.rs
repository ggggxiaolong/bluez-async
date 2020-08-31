// This code was autogenerated with `dbus-codegen-rust --file=specs/org.bluez.AgentManager1.xml --interfaces=org.bluez.AgentManager1 --client=nonblock --methodtype=none`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::nonblock;

pub trait OrgBluezAgentManager1 {
    fn register_agent(&self, agent: dbus::Path, capability: &str) -> nonblock::MethodReply<()>;
    fn unregister_agent(&self, agent: dbus::Path) -> nonblock::MethodReply<()>;
    fn request_default_agent(&self, agent: dbus::Path) -> nonblock::MethodReply<()>;
}

impl<'a, T: nonblock::NonblockReply, C: ::std::ops::Deref<Target=T>> OrgBluezAgentManager1 for nonblock::Proxy<'a, C> {

    fn register_agent(&self, agent: dbus::Path, capability: &str) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.AgentManager1", "RegisterAgent", (agent, capability, ))
    }

    fn unregister_agent(&self, agent: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.AgentManager1", "UnregisterAgent", (agent, ))
    }

    fn request_default_agent(&self, agent: dbus::Path) -> nonblock::MethodReply<()> {
        self.method_call("org.bluez.AgentManager1", "RequestDefaultAgent", (agent, ))
    }
}
