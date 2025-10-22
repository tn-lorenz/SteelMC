use std::{collections::BTreeMap, fs};

use heck::ToSnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Packets {
    version: u32,
    serverbound: BTreeMap<String, Vec<String>>,
    clientbound: BTreeMap<String, Vec<String>>,
}

pub(crate) fn build() -> TokenStream {
    println!("cargo:rerun-if-changed=build_assets/packets.json");

    let packets: Packets =
        serde_json::from_str(&fs::read_to_string("build_assets/packets.json").unwrap())
            .expect("Failed to parse packets.json");
    let version = packets.version;
    let serverbound_consts = parse_packets(
        packets.serverbound,
        Ident::new("Serverbound", Span::call_site()),
    );
    let clientbound_consts = parse_packets(
        packets.clientbound,
        Ident::new("Clientbound", Span::call_site()),
    );

    quote!(
        /// The current Minecraft protocol version. This changes only when the protocol itself is modified.
        pub const CURRENT_MC_PROTOCOL: u32 = #version;

        pub mod serverbound {
            #serverbound_consts
        }

        pub mod clientbound {
            #clientbound_consts
        }
    )
}

pub(crate) fn parse_packets(
    packets: BTreeMap<String, Vec<String>>,
    prefix_ident: Ident,
) -> proc_macro2::TokenStream {
    let mut consts = TokenStream::new();

    for packet in packets {
        let phase = Ident::new(&packet.0.to_snake_case(), Span::call_site());
        let mut inner = TokenStream::new();
        for (id, packet_name) in packet.1.iter().enumerate() {
            let packet_id = id as i32;
            let packet_name = packet_name.replace("/", "_");
            let name = format!("{prefix_ident}_{packet_name}").to_uppercase();
            let name = format_ident!("{}", name);
            inner.extend([quote! {
                pub const #name: i32 = #packet_id;
            }]);
        }
        consts.extend([quote! {
            pub mod #phase {
                #inner
            }
        }]);
    }
    consts
}
