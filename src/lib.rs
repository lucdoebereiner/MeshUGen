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
    n_outputs: usize,
    n_inputs: usize,
    input_offset: usize,
}

#[no_mangle]
pub extern "C" fn new_state(samplerate: c_double) -> *mut UGenState {
    set_sr(samplerate as f64);
    Box::into_raw(Box::new(UGenState {
        output: Vec::new(),
        flow: Vec::new(),
        graph: UGenGraph::new(),
        n_outputs: 0,
        n_inputs: 0,
        input_offset: 0,
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
pub extern "C" fn set_graph(
    state: *mut UGenState,
    buffer: *mut c_float,
    length: c_uint,
    n_channels: c_int,
) {
    unsafe {
        let bytes_buffer_f: &[f32] = slice::from_raw_parts_mut(buffer, length as usize);
        let bytes_buffer: Vec<u8> = bytes_buffer_f.iter().map(|f| *f as u8).collect();

        let json_string = from_utf8(&bytes_buffer).unwrap();

        (*state).output = vec![0.0; 2];
        (*state).flow = vec![];

        (*state).graph =
            UGenGraph::from_json_string(json_string.to_string(), &mut (*state).flow).unwrap();

        (*state).n_inputs = (*state).graph.number_of_inputs();
        (*state).n_outputs = n_channels as usize;
        (*state).input_offset = 3 + n_channels as usize + 12;
        // (*state)
        //     .graph
        //     .offset_sound_ins(3 + n_channels as usize + 12);
        (*state).graph.reset_outputs();
    }
}

#[no_mangle]
pub extern "C" fn process(
    state: &mut UGenState,
    sc_in: *mut *mut c_float,
    sc_out: *mut *mut c_float,
    sc_nsamples: c_int,
) {
    let n_out = (*state).n_outputs;
    let n_in = (*state).n_inputs;
    let total_n_in = n_in + 2 + n_out + 12;

    unsafe {
        let out_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_out, n_out as usize);
        let in_buffer: &mut [*mut f32] = slice::from_raw_parts_mut(sc_in, total_n_in);
        let mut out_channels: Vec<&mut [f32]> = Vec::new();
        for c in 0..n_out {
            out_channels.push(slice::from_raw_parts_mut(
                out_buffer[c],
                sc_nsamples as usize,
            ));
        }
        let mut in_channels: Vec<&mut [f32]> = Vec::new();
        if n_in > 0 {
            for c in 0..total_n_in {
                in_channels.push(slice::from_raw_parts_mut(
                    in_buffer[c],
                    sc_nsamples as usize,
                ));
            }
        }

        (*state).graph.set_edge_fac(in_channels[2][0] as f64);

        (*state)
            .graph
            .set_edges_weight(in_channels[3 + n_out][0] as f64, 4, 0);
        (*state)
            .graph
            .set_edges_weight(in_channels[4 + n_out][0] as f64, 4, 1);
        (*state)
            .graph
            .set_edges_weight(in_channels[5 + n_out][0] as f64, 4, 2);
        (*state)
            .graph
            .set_edges_weight(in_channels[6 + n_out][0] as f64, 4, 3);

        (*state)
            .graph
            .set_edges_delay(in_channels[7 + n_out][0] as f64, 4, 0);
        (*state)
            .graph
            .set_edges_delay(in_channels[8 + n_out][0] as f64, 4, 1);
        (*state)
            .graph
            .set_edges_delay(in_channels[9 + n_out][0] as f64, 4, 2);
        (*state)
            .graph
            .set_edges_delay(in_channels[10 + n_out][0] as f64, 4, 3);

        (*state)
            .graph
            .set_edges_lp_freq(in_channels[11 + n_out][0] as f64, 4, 0);
        (*state)
            .graph
            .set_edges_lp_freq(in_channels[12 + n_out][0] as f64, 4, 1);
        (*state)
            .graph
            .set_edges_lp_freq(in_channels[13 + n_out][0] as f64, 4, 2);
        (*state)
            .graph
            .set_edges_lp_freq(in_channels[14 + n_out][0] as f64, 4, 3);

        //        println!("last freq: {}", in_channels[14][0]);

        for channel in 0..n_out {
            (*state)
                .graph
                .set_steto_output_channel(channel, in_channels[3 + channel][0] as f64);
            // println!(
            //     "steto for channel {} is {}",
            //     channel,
            //     in_channels[3 + channel][0] as f64
            // );
        }

        for i in 0..sc_nsamples {
            let inputs: Vec<f64> = in_channels
                .iter()
                .skip((*state).input_offset)
                .map(|input| input[i as usize] as f64)
                .collect();

            (*state)
                .graph
                .process(&(*state).flow, inputs.as_slice(), &mut (*state).output);
            for channel in 0..n_out {
                out_channels[channel][i as usize] = (*state).output[channel] as f32;
            }
        }
    }
}
