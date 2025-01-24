use crate::app::AppSection;
use crate::signing_key::{CryptoKeyContainer};
use derive_more::{Deref, DerefMut};
use egui::Ui;

type CryptoKey = (); // TODO

#[derive(Deref, DerefMut, Clone, Debug)]
pub struct SigningKeyArray<const N: usize> {
    #[deref]
    #[deref_mut]
    keys: [CryptoKeyContainer; N],
}

impl<const N: usize> Default for SigningKeyArray<N> {
    fn default() -> Self {
        SigningKeyArray {
            keys: core::array::from_fn(|_| Default::default()),
        }
    }
}

impl<const N: usize> AppSection for SigningKeyArray<N> {
    fn draw_and_update(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.group(|ui| {
            for key in self.keys.iter_mut() {
                key.draw_and_update(ctx, ui);
            }
        });
    }
}
