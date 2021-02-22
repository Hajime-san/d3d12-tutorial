use bindings::{
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::win32::system_services as system_services,
    windows::win32::dxgi as dxgi,
    windows::win32::direct3d12 as direct3d12,
    windows::win32::direct3d11 as direct3d11,
    windows::win32::windows_programming as windows_programming,
    windows::BOOL,
    windows::Interface,
};

use std::{
    ffi,
    ptr,
    mem,
};
// use std::mem;

pub mod util;
pub mod win;
pub mod d3d;

const DEBUG: bool = true;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn main() {

    let class_name = util::utf16_to_vec("D3D12Sample");

    let h_wnd = win::create_window(&class_name, WINDOW_WIDTH, WINDOW_HEIGHT);

    let dxgi_factory = d3d::create_dxgi_factory2::<dxgi::IDXGIFactory6>(system_services::DXGI_CREATE_FACTORY_DEBUG).unwrap();

    // dxgi_factory = d3d::create_dxgi_factory1::<IDXGIFactory1>().unwrap();

    /// enable debug layer
    d3d::enable_debug_layer(&DEBUG);

    /// create device
    let d3d12_device = d3d::create_d3d12_device(None).unwrap();

    /// create command list, allocator
    let command_allocator = d3d::create_command_allocator(&d3d12_device, direct3d12::D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT).unwrap();

    // create commnad queue
    let command_queue_desc = direct3d12::D3D12_COMMAND_QUEUE_DESC {
        flags : direct3d12::D3D12_COMMAND_QUEUE_FLAGS::D3D12_COMMAND_QUEUE_FLAG_NONE,
        node_mask : 0,
        priority : direct3d12::D3D12_COMMAND_QUEUE_PRIORITY::D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
        r#type : direct3d12::D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT,
    };

    let command_queue = d3d::create_command_queue(&d3d12_device, &command_queue_desc).unwrap();

    // create swapchain
    let swapchain_desc1 = dxgi::DXGI_SWAP_CHAIN_DESC1 {
        width : WINDOW_WIDTH as u32,
        height : WINDOW_HEIGHT as u32,
        format : dxgi::DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM,
        stereo : BOOL(0),
        sample_desc: dxgi::DXGI_SAMPLE_DESC {
            count : 1,
            quality : 0,
        },
        buffer_usage : dxgi::DXGI_USAGE_BACK_BUFFER,
        buffer_count : 2,
        scaling : dxgi::DXGI_SCALING::DXGI_SCALING_STRETCH,
        swap_effect : dxgi::DXGI_SWAP_EFFECT::DXGI_SWAP_EFFECT_FLIP_DISCARD,
        alpha_mode : dxgi::DXGI_ALPHA_MODE::DXGI_ALPHA_MODE_UNSPECIFIED,
        flags : 0,
    };

    let mut swapchain = d3d::create_swap_chain_for_hwnd(&dxgi_factory, &command_queue, h_wnd, &swapchain_desc1, ptr::null_mut(), None).unwrap();

    // create discriptor heap
    let heap_desc = direct3d12::D3D12_DESCRIPTOR_HEAP_DESC {
        r#type : direct3d12::D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
        node_mask : 0,
        num_descriptors : 2,
        flags : direct3d12::D3D12_DESCRIPTOR_HEAP_FLAGS::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
    };

    let rtv_heaps = d3d::create_descriptor_heap(&d3d12_device, &heap_desc).unwrap();

    // bind render target view heap to swap chain buffer
    let back_buffers = d3d::create_back_buffer(&d3d12_device, &mut swapchain, swapchain_desc1, &rtv_heaps, None);

    let fence = d3d::create_fence(&d3d12_device, 0, direct3d12::D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_NONE).unwrap();

     // create vertices
     let vertices = vec![
        d3d::Vertex {
            position: d3d::XMFLOAT3 { x: -0.5, y: -0.7, z: 0.0 },
        },
        d3d::Vertex {
            position: d3d::XMFLOAT3 { x: 0.0, y: 0.7, z: 0.0 },
        },
        d3d::Vertex {
            position: d3d::XMFLOAT3 { x: 0.5, y: -0.7, z: 0. },
        },
    ];

    // create vertex buffer

    // settings of vertex heap
    let vertex_buffer_heap_prop = direct3d12::D3D12_HEAP_PROPERTIES {
        r#type : direct3d12::D3D12_HEAP_TYPE::D3D12_HEAP_TYPE_UPLOAD,
        cpu_page_property : direct3d12::D3D12_CPU_PAGE_PROPERTY::D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
        memory_pool_preference : direct3d12::D3D12_MEMORY_POOL::D3D12_MEMORY_POOL_UNKNOWN,
        creation_node_mask: 1,
        visible_node_mask: 1,
    };

    // vertex buffer object
    let vertex_buffer_resource_desc = direct3d12::D3D12_RESOURCE_DESC {
        dimension : direct3d12::D3D12_RESOURCE_DIMENSION::D3D12_RESOURCE_DIMENSION_BUFFER,
        alignment: 0,
        width : (std::mem::size_of_val(&vertices) * &vertices.len()) as u64,
        height : 1,
        depth_or_array_size : 1,
        mip_levels : 1,
        format : dxgi::DXGI_FORMAT::DXGI_FORMAT_UNKNOWN,
        sample_desc: dxgi::DXGI_SAMPLE_DESC {
            count : 1,
            quality: 0,
        },
        flags : direct3d12::D3D12_RESOURCE_FLAGS::D3D12_RESOURCE_FLAG_NONE,
        layout : direct3d12::D3D12_TEXTURE_LAYOUT::D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    };

    let comitted_resource = d3d::CommittedResource {
        p_heap_properties: &vertex_buffer_heap_prop,
        heap_flags: direct3d12::D3D12_HEAP_FLAGS::D3D12_HEAP_FLAG_NONE,
        p_resource_desc: &vertex_buffer_resource_desc,
        initial_resource_state: direct3d12::D3D12_RESOURCE_STATES::D3D12_RESOURCE_STATE_GENERIC_READ,
        p_optimized_clear_value: std::ptr::null_mut(),
    };

    // create indices
    let indices = vec![
        0, 1, 2,
        2, 1, 3
    ];

    // create vertex resources
    let vertex_buffer = d3d::create_vertex_buffer_resources(&d3d12_device, &comitted_resource, &vertices);

    let index_buffer = d3d::create_index_buffer_resources(&d3d12_device, &comitted_resource, &indices);

    // create shader object
    let vertex_shader_blob = d3d::create_shader_resource("shaders\\VertexShader.hlsl", "BasicVS", "vs_5_0").unwrap();
    let pixel_shader_blob = d3d::create_shader_resource("shaders\\PixelShader.hlsl", "BasicPS", "ps_5_0").unwrap();

    // vertex layout
    let mut input_element: [direct3d12::D3D12_INPUT_ELEMENT_DESC; 1] = [
        direct3d12::D3D12_INPUT_ELEMENT_DESC {
            semantic_name: ffi::CString::new("POSITION").unwrap().into_raw() as *mut i8,
            semantic_index: 0,
            format: dxgi::DXGI_FORMAT::DXGI_FORMAT_R32G32B32_FLOAT,
            input_slot: 0,
            aligned_byte_offset: direct3d12::D3D12_APPEND_ALIGNED_ELEMENT,
            input_slot_class: direct3d12::D3D12_INPUT_CLASSIFICATION::D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
            instance_data_step_rate: 0,
        },
    ];

    // create root signature
    let root_signature = d3d::create_root_signature(&d3d12_device).unwrap();

    // create graphics pipeline
    let mut gr_pipeline = direct3d12::D3D12_GRAPHICS_PIPELINE_STATE_DESC::default();
    // let mut gr_pipeline: direct3d12::D3D12_GRAPHICS_PIPELINE_STATE_DESC = unsafe { mem::zeroed() };

    // set shader
    gr_pipeline.p_root_signature = Some(root_signature);
    gr_pipeline.vs.p_shader_bytecode = unsafe { vertex_shader_blob.GetBufferPointer() };
    gr_pipeline.vs.bytecode_length = unsafe { vertex_shader_blob.GetBufferSize() };
    gr_pipeline.ps.p_shader_bytecode = unsafe { pixel_shader_blob.GetBufferPointer() };
    gr_pipeline.ps.bytecode_length = unsafe { pixel_shader_blob.GetBufferSize() };

    // sample mask
    gr_pipeline.sample_mask = direct3d12::D3D12_DEFAULT_SAMPLE_MASK;

    // culling, filling
    gr_pipeline.rasterizer_state.cull_mode = direct3d12::D3D12_CULL_MODE::D3D12_CULL_MODE_NONE;
    gr_pipeline.rasterizer_state.fill_mode = direct3d12::D3D12_FILL_MODE::D3D12_FILL_MODE_SOLID;
    gr_pipeline.rasterizer_state.depth_clip_enable = BOOL(1);

    // blend mode
    gr_pipeline.blend_state.alpha_to_coverage_enable = BOOL(0);
    gr_pipeline.blend_state.independent_blend_enable = BOOL(0);

    // render target blend settings
    let mut render_target_blend_desc = direct3d12::D3D12_RENDER_TARGET_BLEND_DESC::default();
    render_target_blend_desc.blend_enable = BOOL(0);
    render_target_blend_desc.logic_op_enable = BOOL(0);
    render_target_blend_desc.render_target_write_mask = direct3d12::D3D12_COLOR_WRITE_ENABLE::D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8;

    // gr_pipeline.blend_state.render_target[0] = render_target_blend_desc;

    // bind input layout
    gr_pipeline.input_layout.p_input_element_descs = &mut input_element[0];
    gr_pipeline.input_layout.num_elements = input_element.len() as u32;

    // way to express triangle
    gr_pipeline.ib_strip_cut_value = direct3d12::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE::D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED;

    // primitive topology setting
    gr_pipeline.primitive_topology_type = direct3d12::D3D12_PRIMITIVE_TOPOLOGY_TYPE::D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE;

    // render target settings
    gr_pipeline.num_render_targets = 1;
    // gr_pipeline.rtv_formats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;

    // anti aliasing
    gr_pipeline.rasterizer_state.multisample_enable = BOOL(0);
    gr_pipeline.sample_desc.count = 1;
    gr_pipeline.sample_desc.quality = 0;

    // create grahphics pipeline state object
    // let pipeline_state = d3d::create_pipeline_state(&d3d12_device, &gr_pipeline).unwrap();

    // viewport setting
    let viewport = d3d::set_viewport(WINDOW_WIDTH, WINDOW_HEIGHT);

    // scissor rectangle setting
    // let scissor_rect = d3d::set_scissor_rect(WINDOW_WIDTH, WINDOW_HEIGHT);

    let command_list = d3d::create_command_list(
            &d3d12_device,
            0,
            direct3d12::D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT,
            &command_allocator,
            None
        ).unwrap();

    let mut current_frame = 0;
    let clear_color: [f32; 4] = [ 1.0, 1.0, 0.0, 1.0 ];

    win::show_window(h_wnd);

    let mut message = win::creat_message();

    while unsafe { windows_and_messaging::GetMessageW(&mut message, windows_and_messaging::HWND(0), 0, 0).into() } {
        unsafe { windows_and_messaging::DispatchMessageW(&mut message); }

        // increment frame
        current_frame += 1;

        // get back buffer index
        // let back_buffers_index = swapchain.cast::<dxgi::IDXGISwapChain4>().unwrap().GetCurrentBackBufferIndex();

        // // create resource barrier

        // let mut barrier_desc = direct3d12::D3D12_RESOURCE_BARRIER {
        //     r#type : direct3d12::D3D12_RESOURCE_BARRIER_TYPE::D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        //     flags : direct3d12::D3D12_RESOURCE_BARRIER_FLAGS::D3D12_RESOURCE_BARRIER_FLAG_NONE,
        //     anonymous: unsafe { std::mem::zeroed() },
        // };
        // *{ barrier_desc.transition } =
        //     direct3d12::D3D12_RESOURCE_TRANSITION_BARRIER {
        //     p_resource : back_buffers[back_buffers_index as usize],
        //     subresource : direct3d12::D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
        //     state_before : direct3d12::D3D12_RESOURCE_STATES::D3D12_RESOURCE_STATE_PRESENT,
        //     state_after : direct3d12::D3D12_RESOURCE_STATES::D3D12_RESOURCE_STATE_RENDER_TARGET,
        // };

        // command_list.ResourceBarrier(1, &barrier_desc);

        // command_list.SetPipelineState(&pipeline_state);

        // // set render target
        // let rtv_heap_start = unsafe {
        //     let mut tmp = direct3d12::D3D12_CPU_DESCRIPTOR_HANDLE::default();
        //     rtv_heaps.GetCPUDescriptorHandleForHeapStart(&mut tmp);
        //     tmp
        // };

        // rtv_heap_start.ptr += (back_buffers_index * d3d12_device.GetDescriptorHandleIncrementSize(direct3d12::D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_RTV)) as usize;

        // command_list.OMSetRenderTargets(
        //     1,
        //     &rtv_heap_start,
        //     BOOL(0),
        //     std::ptr::null_mut()
        // );

        // // clear render target
        // command_list.ClearRenderTargetView(rtv_heap_start, &clear_color as *const _, 0, std::ptr::null_mut());

        // // draw call
        // command_list.RSSetViewports(1, &viewport);
        // // command_list.RSSetScissorRects(1, &scissor_rect);
        // command_list.SetComputeRootSignature(&root_signature);
        // command_list.IASetPrimitiveTopology(direct3d11::D3D_PRIMITIVE_TOPOLOGY::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        // command_list.IASetVertexBuffers(0, 1, &vertex_buffer.buffer_view);
        // command_list.IASetIndexBuffer(&index_buffer.buffer_view);
        // command_list.DrawInstanced(3, 1, 0, 0);

        // // swap barrier state
        // barrier_desc.transition.state_before = direct3d12::D3D12_RESOURCE_STATES::D3D12_RESOURCE_STATE_RENDER_TARGET;
        // barrier_desc.transition.state_after = direct3d12::D3D12_RESOURCE_STATES::D3D12_RESOURCE_STATE_PRESENT;
        // command_list.ResourceBarrier(1, &barrier_desc);

        // // run commands
        // command_list.Close();

        // let cmd_list_array = [ command_list.cast::<direct3d12::ID3D12CommandList>() ];

        // command_queue.ExecuteCommandLists(1, &cmd_list_array[0]);

        // // handle fence
        // command_queue.Signal(fence, current_frame);

        // if fence.GetCompletedValue() != current_frame {
        //     let event = system_services::CreateEventW(ptr::null_mut(), BOOL(0), BOOL(0), ptr::null_mut());

        //     fence.SetEventOnCompletion(current_frame, event);

        //     system_services::WaitForSingleObject(event, system_services::INFINITE);

        //     windows_programming::CloseHandle(event);
        // }

        // command_allocator.Reset();

        // command_list.Reset(command_allocator, ptr::null_mut());


        // // swap buffer
        // swapchain.Present(1, 0);

    }
}
