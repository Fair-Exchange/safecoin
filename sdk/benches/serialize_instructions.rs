#![feature(test)]

extern crate test;
use bincode::{deserialize, serialize};
use safecoin_sdk::instruction::{AccountMeta, Instruction};
use safecoin_sdk::message::Message;
use safecoin_sdk::pubkey;
use safecoin_sdk::sysvar::instructions;
use test::Bencher;

fn make_instructions() -> Vec<Instruction> {
    let meta = AccountMeta::new(pubkey::new_rand(), false);
    let inst = Instruction::new_with_bincode(pubkey::new_rand(), &[0; 10], vec![meta; 4]);
    vec![inst; 4]
}

const DEMOTE_PROGRAM_WRITE_LOCKS: bool = true;

#[bench]
fn bench_bincode_instruction_serialize(b: &mut Bencher) {
    let instructions = make_instructions();
    b.iter(|| {
        test::black_box(serialize(&instructions).unwrap());
    });
}

#[bench]
fn bench_manual_instruction_serialize(b: &mut Bencher) {
    let instructions = make_instructions();
    let message = Message::new(&instructions, None);
    b.iter(|| {
        test::black_box(message.serialize_instructions(DEMOTE_PROGRAM_WRITE_LOCKS));
    });
}

#[bench]
fn bench_bincode_instruction_deserialize(b: &mut Bencher) {
    let instructions = make_instructions();
    let serialized = serialize(&instructions).unwrap();
    b.iter(|| {
        test::black_box(deserialize::<Vec<Instruction>>(&serialized).unwrap());
    });
}

#[bench]
fn bench_manual_instruction_deserialize(b: &mut Bencher) {
    let instructions = make_instructions();
    let message = Message::new(&instructions, None);
    let serialized = message.serialize_instructions(DEMOTE_PROGRAM_WRITE_LOCKS);
    b.iter(|| {
        for i in 0..instructions.len() {
            test::black_box(instructions::load_instruction_at(i, &serialized).unwrap());
        }
    });
}

#[bench]
fn bench_manual_instruction_deserialize_single(b: &mut Bencher) {
    let instructions = make_instructions();
    let message = Message::new(&instructions, None);
    let serialized = message.serialize_instructions(DEMOTE_PROGRAM_WRITE_LOCKS);
    b.iter(|| {
        test::black_box(instructions::load_instruction_at(3, &serialized).unwrap());
    });
}
