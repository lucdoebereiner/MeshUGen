//use std::f32;
use petgraph::stable_graph::NodeIndex;
use processgraph::process::set_sr;
use processgraph::*;
use std::os::raw::{c_double, c_float, c_int, c_uint};
use std::slice;
use std::str::from_utf8;

pub struct UGenState {
    output: Vec<f64>,
    flow: Vec<NodeIndex>,
    graph: UGenGraph,
}

#[no_mangle]
pub extern "C" fn new_state(samplerate: c_double) -> *mut UGenState {
    set_sr(samplerate as f64);
    Box::into_raw(Box::new(UGenState {
        output: Vec::new(),
        flow: Vec::new(),
        graph: UGenGraph::new(),
    }))
}

#[no_mangle]
pub extern "C" fn state_free(state: *mut UGenState) {
    if state.is_null() {
        return;
    }
    // here it goes out of scope and thus gets dropped by rust
    unsafe { Box::from_raw(state) };
}

#[no_mangle]
pub extern "C" fn set_graph(state: *mut UGenState, buffer: *mut c_float, length: c_uint) {
    unsafe {
        let bytes_buffer_f: &[f32] = slice::from_raw_parts_mut(buffer, length as usize);
        let bytes_buffer: Vec<u8> = bytes_buffer_f.iter().map(|f| *f as u8).collect();

        // let buffer_bytes: &[u8] =
        //     std::slice::from_raw_parts(buffer as *const u8, (length as usize) * 4);

        // let string_bytes: Vec<u8> = buffer_bytes
        //     .chunks(4)
        //     .map(|chunk| {
        //         let rev_chunk: Vec<u8> = chunk.iter().rev().map(|b| *b).collect();
        //         rev_chunk
        //     })
        //     .flatten()
        //     .collect();

        // let json_string = from_utf8(&string_bytes);
        let json_string = from_utf8(&bytes_buffer).unwrap();

        //        *graph = UGenGraph::new();

        (*state).output = vec![0.0; 2];
        (*state).flow = vec![];

        (*state).graph =
            UGenGraph::from_json_string(json_string.to_string(), &mut (*state).flow).unwrap();

        (*state).graph.offset_sound_ins(2);
        // (*state).graph.init_after_deserialization();
        // (*state)
        //     .graph
        //     .update_connections_and_flow(&mut (*state).flow);

        //     println!("{:?}", json_string);

        // let input1 = soundinput(1);
        // let input2 = soundinput(2);
        // let mut ugen1 = sinosc(10.0);
        // let mut ugen2 = sinosc(0.0);
        // //        let mut ugen2 = sinosc(200.0);
        // ugen1.set_output(0, 1.0);
        // ugen2.set_output(1, 1.0);
        // //        ugen2.set_output(1, 1.0);
        // let idx1 = (*state).graph.add(input1);
        // let idx2 = (*state).graph.add(input2);
        // let idx3 = (*state).graph.add(ugen1);
        // let idx4 = (*state).graph.add(ugen2);
        // (*state).graph.connect(idx1, idx3, Connection::new(0, 1.0));
        // (*state).graph.connect(idx2, idx4, Connection::new(0, 1.0));
        // (*state).output = vec![0.0; 2];
        // (*state).flow = vec![];
        // (*state)
        //     .graph
        //     .update_connections_and_flow(&mut (*state).flow);

        // println!(
        //     "outs: {}, ins: {:?}",
        //     (*state).graph.number_of_outputs(),
        //     (*state).graph.number_of_inputs()
        // );
    }
}

#[no_mangle]
pub extern "C" fn process(
    state: &mut UGenState,
    sc_in: *mut *mut c_float,
    sc_out: *mut *mut c_float,
    sc_nsamples: c_int,
) {
    let n_out = (*state).graph.number_of_outputs();
    let n_in = (*state).graph.number_of_inputs();

    unsafe {
        let out_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_out, n_out as usize);
        let in_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_in, n_in);
        let mut out_channels: Vec<&mut [f32]> = Vec::new();
        for c in 0..n_out {
            out_channels.push(slice::from_raw_parts_mut(
                out_buffer[c],
                sc_nsamples as usize,
            ));
        }
        let mut in_channels: Vec<&mut [f32]> = Vec::new();
        if n_in > 0 {
            for c in 0..n_in {
                in_channels.push(slice::from_raw_parts_mut(
                    in_buffer[c],
                    sc_nsamples as usize,
                ));
            }
        }

        for i in 0..sc_nsamples {
            let inputs: Vec<f64> = in_channels
                .iter()
                .map(|input| input[i as usize] as f64)
                .collect();
            //            println!("fac:{}", inputs[1]);
            (*state).graph.set_edge_fac(inputs[1]);
            (*state)
                .graph
                .process(&(*state).flow, inputs.as_slice(), &mut (*state).output);
            for channel in 0..n_out {
                out_channels[channel][i as usize] = (*state).output[channel] as f32;
            }
        }
    }
}
