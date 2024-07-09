

pub mod software {
  pub mod amazon {
    pub mod cryptography {
      pub mod internaldafny {
        pub mod StormTrackingCMC {
          use crate::*;
          pub use crate::storm_tracker::internal_StormTrackingCMC::*;
        }
        pub mod SynchronizedLocalCMC {
          use crate::*;
          pub use crate::local_cmc::internal_SynchronizedLocalCMC::*;
        }
      }
    }
  }
}