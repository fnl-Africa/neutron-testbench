use neutron_qx86_hypervisor::*;
use neutron_qx86_hypervisor::hypervisor::*;
use qx86::vm::*;

#[derive(Clone, Debug, Default)]
pub struct TestbenchAPI{
    sccs: Vec<Vec<u8>>,
    pub context: NeutronContext
}

impl NeutronAPI for TestbenchAPI{
    fn get_context(&self) -> &NeutronContext{
        &self.context
    }
    fn push_sccs(&mut self, data: &Vec<u8>) -> Result<(), NeutronError>{
        self.sccs.push(data.clone());
        Ok(())
    }
    fn pop_sccs(&mut self, data: &mut Vec<u8>) -> Result<(), NeutronError>{
        let p = self.sccs.pop().ok_or(NeutronError::RecoverableFailure)?;
        data.resize(p.len(), 0);
        data.copy_from_slice(&p);
        Ok(())
    }
    fn pop_sccs_toss(&mut self) -> Result<(), NeutronError>{
        if self.sccs.len() == 0{
            Err(NeutronError::RecoverableFailure)
        }else{
            let _ = self.sccs.remove(self.sccs.len() - 1);
            Ok(())
        }
    }
    fn peek_sccs(&mut self, data: &mut Vec<u8>) -> Result<(), NeutronError>{
        if self.sccs.len() == 0{
            Err(NeutronError::RecoverableFailure)
        }else{
            let p = &self.sccs[self.sccs.len() - 1];
            data.copy_from_slice(p);
            Ok(())
        }
    }
    fn peek_sccs_size(&mut self) -> Result<usize, NeutronError>{
        Ok(self.sccs.len())
    }

    fn log_error(&mut self, msg: &str){
        println!("ERROR: {}", msg);
    }
    fn log_info(&mut self, msg: &str){
        println!("INFO: {}", msg);
    }
    fn log_debug(&mut self, msg: &str){
        println!("DEBUG: {}", msg);
    }
}
impl NeutronHypervisor for TestbenchAPI{}
impl Hypervisor for TestbenchAPI{}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
}
