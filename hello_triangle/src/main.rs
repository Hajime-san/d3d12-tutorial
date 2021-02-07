use bindings::{
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::win32::system_services as system_services,
    windows::win32::dxgi as dxgi,
    windows::win32::direct3d12 as direct3d12,
    windows::BOOL,
};

use std::{ptr};
// use std::mem;

pub mod util;
pub mod win;
pub mod d3d;

const DEBUG: bool = true;
const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

type INDICES = [u32; 6];

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
        priority : 0,
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
    // let input_element: [D3D12_INPUT_ELEMENT_DESC; 1] = [
    //     D3D12_INPUT_ELEMENT_DESC {
    //         SemanticName: CString::new("POSITION").unwrap().into_raw(),
    //         SemanticIndex: 0,
    //         Format: DXGI_FORMAT_R32G32B32_FLOAT,
    //         InputSlot: 0,
    //         AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
    //         InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
    //         InstanceDataStepRate: 0,
    //     },
    // ];

    // let command_list = d3d::create_command_list(d3d12_device, 0, D3D12_COMMAND_LIST_TYPE::D3D12_COMMAND_LIST_TYPE_DIRECT, command_allocator, ptr::null_mut()).unwrap();

    win::show_window(h_wnd);

    let mut message = win::creat_message();

    while unsafe { windows_and_messaging::GetMessageW(&mut message, windows_and_messaging::HWND(0), 0, 0).into() } {
        unsafe { windows_and_messaging::DispatchMessageW(&mut message); }

    }
}
