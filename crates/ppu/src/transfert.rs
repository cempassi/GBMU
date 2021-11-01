mod fetcher;
use futures::future::FutureExt;

use crate::futures::Writer;
use futures::{pending, select};
use shared::Error;

use crate::{Field, Ppu};
use fetcher::Fetcher;

#[allow(dead_code)]
pub struct Pixel {
    ppu: Ppu,
}

impl Pixel {
    pub fn transfert(ppu: Ppu) -> Self {
        Self { ppu }
    }

    pub async fn start(self) -> Result<u8, Error> {
        let ly = self.ppu.borrow().registers.coordinates.get(Field::Ly);
        let line = ly % 8;
        let mut id = 0;
        let id_address = 0x9800 + ((ly as u16 / 8) * 32);

        let fetcher = Fetcher::new(self.ppu.clone(), id_address, id).fetch(line);
        let mut fetching = Box::pin(fetcher).fuse();
        let mut writer = Box::pin(Writer::new(&self.ppu)).fuse();

        // Mot sure about the timings here, needs further investigation
        while !self.ppu.borrow_mut().line_finished() {
            select! {
                _ = fetching => {
                    id += 1;
                    let fetcher = Fetcher::new(self.ppu.clone(), id_address,  id).fetch(line);
                    fetching =  Box::pin(fetcher).fuse();
                },
                _ = writer => {
                writer =  Box::pin(Writer::new(&self.ppu)).fuse();
                    },
                complete => {
                    id += 1;
                    let fetcher = Fetcher::new(self.ppu.clone(), id_address,  id).fetch(line);
                    fetching =  Box::pin(fetcher).fuse();
                    writer =  Box::pin(Writer::new(&self.ppu)).fuse();
                }
            };
            pending!();
        }
        Ok(0)
    }
}
