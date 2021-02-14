use bindings::{
    windows::win32::dxgi as dxgi,
    windows::win32::direct3d12 as direct3d12,
    windows::win32::direct3d11 as direct3d11,
    windows::win32::direct3d_hlsl as direct3d_hlsl,
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::Interface,
    windows::IUnknown,
    windows::ErrorCode,
    windows::Error as WinError,
    windows::Abi,
    windows::Result as WinResult,
};

use std::{
    ptr,
    mem,
    ffi,
    string,
};

use crate::util;

#[derive(Debug, Clone)]
pub struct CommittedResource {
    pub p_heap_properties: *const direct3d12::D3D12_HEAP_PROPERTIES,
    pub heap_flags: direct3d12::D3D12_HEAP_FLAGS,
    pub p_resource_desc: *const direct3d12::D3D12_RESOURCE_DESC,
    pub initial_resource_state: direct3d12::D3D12_RESOURCE_STATES,
    pub p_optimized_clear_value: *const direct3d12::D3D12_CLEAR_VALUE
}

#[derive(Debug, Clone)]
pub struct BufferResources<T> {
    pub buffer_view: T,
    pub buffer_object: WinResult<direct3d12::ID3D12Resource>,
}
#[derive(Debug, Clone, Copy)]
pub struct XMFLOAT3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
#[derive(Debug, Clone, Copy)]
pub struct XMFLOAT2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: XMFLOAT3,
    // pub uv: XMFLOAT2,
}

pub fn create_dxgi_factory1<T:Interface>() -> WinResult<T> {
    unsafe {
        let mut dxfactory: Option<T> = None;
        dxgi::CreateDXGIFactory1(
            &T::IID,
            dxfactory.set_abi()
        )
        .and_some(dxfactory)
    }
}

pub fn create_dxgi_factory2<T:Interface>(flags: u32) -> WinResult<T> {
    unsafe {
        let mut dxfactory: Option<T> = None;
        dxgi::CreateDXGIFactory2(
            flags,
            &T::IID,
            dxfactory.set_abi()
        )
        .and_some(dxfactory)
    }
}

pub fn create_d3d12_device(p_adapter: Option<IUnknown>) -> WinResult<direct3d12::ID3D12Device> {
    let levels: [direct3d11::D3D_FEATURE_LEVEL; 4] = [
        direct3d11::D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_1,
        direct3d11::D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
        direct3d11::D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_1,
        direct3d11::D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0
    ];

    let mut result: ErrorCode = ErrorCode::E_POINTER;
    let mut d3d_device: Option<direct3d12::ID3D12Device> = None;

    for lv in levels.iter() {
        unsafe {
            result = direct3d12::D3D12CreateDevice(
                p_adapter.clone(),
                *lv,
                &direct3d12::ID3D12Device::IID,
                d3d_device.set_abi()
            )
        };

        if result == ErrorCode::S_OK {
            break;
        }
    }

    result.and_some(d3d_device)
}

pub fn enable_debug_layer(is_debug: &bool) {
    if !is_debug {
        return;
    }

    unsafe {
        let mut debug_interface: Option<direct3d12::ID3D12Debug> = None;
        direct3d12::D3D12GetDebugInterface(
            &direct3d12::ID3D12Debug::IID,
            debug_interface.set_abi()
        )
        .and_some(debug_interface)
        .as_ref().unwrap()
        .EnableDebugLayer();
    }
}

pub fn create_command_allocator(device: &direct3d12::ID3D12Device, r#type: direct3d12::D3D12_COMMAND_LIST_TYPE) -> WinResult<direct3d12::ID3D12CommandAllocator> {
    unsafe {
        let mut command_allocator: Option<direct3d12::ID3D12CommandAllocator> = None;
        device.
            CreateCommandAllocator(
                r#type,
                &direct3d12::ID3D12CommandAllocator::IID,
                command_allocator.set_abi()
            )
            .and_some(command_allocator)
    }
}

pub fn create_command_list(
    device: &direct3d12::ID3D12Device,
    node_mask: u32,
    r#type: direct3d12::D3D12_COMMAND_LIST_TYPE,
    p_command_allocator: &direct3d12::ID3D12CommandAllocator,
    p_initial_state: Option<&direct3d12::ID3D12PipelineState>)
     -> WinResult<direct3d12::ID3D12GraphicsCommandList> {
    unsafe {
        let mut command_list: Option<direct3d12::ID3D12GraphicsCommandList> = None;
        device.
            CreateCommandList(
                node_mask,
                r#type,
                p_command_allocator,
                p_initial_state.and(None),
                &direct3d12::ID3D12GraphicsCommandList::IID,
                command_list.set_abi()
            )
            .and_some(command_list)
    }
}

pub fn create_command_queue(device: &direct3d12::ID3D12Device, p_desc: *const direct3d12::D3D12_COMMAND_QUEUE_DESC) -> WinResult<direct3d12::ID3D12CommandQueue> {
    unsafe {
        let mut command_queue: Option<direct3d12::ID3D12CommandQueue> = None;
        device.
            CreateCommandQueue(
                p_desc,
                &direct3d12::ID3D12CommandQueue::IID,
                command_queue.set_abi()
            )
            .and_some(command_queue)
    }
}

pub fn create_swap_chain_for_hwnd(
    dxgi_factory: &dxgi::IDXGIFactory6,
    p_device: &direct3d12::ID3D12CommandQueue,
    h_wnd: windows_and_messaging::HWND,
    p_desc: *const dxgi::DXGI_SWAP_CHAIN_DESC1,
    p_fullscreen_desc: *const dxgi::DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    p_restrict_to_output: Option<dxgi::IDXGIOutput>)
    -> WinResult<dxgi::IDXGISwapChain1> {
    unsafe {
        let mut swap_chain: Option<dxgi::IDXGISwapChain1> = None;
        dxgi_factory.
            CreateSwapChainForHwnd(
                p_device,
                h_wnd,
                p_desc,
                p_fullscreen_desc,
                p_restrict_to_output,
                &mut swap_chain
            )
            .and_some(swap_chain)
    }
}

pub fn create_descriptor_heap(device: &direct3d12::ID3D12Device, p_descriptor_heap_desc: *const direct3d12::D3D12_DESCRIPTOR_HEAP_DESC) -> WinResult<direct3d12::ID3D12DescriptorHeap> {
    unsafe {
        let mut descriptor_heap: Option<direct3d12::ID3D12DescriptorHeap> = None;
        device.
        CreateDescriptorHeap(
            p_descriptor_heap_desc,
            &direct3d12::ID3D12DescriptorHeap::IID,
            descriptor_heap.set_abi()
        )
        .and_some(descriptor_heap)
    }
}

pub fn create_back_buffer(
    device: &direct3d12::ID3D12Device,
    swapchain: &mut dxgi::IDXGISwapChain1,
    swapchain_desc: dxgi::DXGI_SWAP_CHAIN_DESC1,
    descriotor_heap: &direct3d12::ID3D12DescriptorHeap,
    p_desc: Option<*const direct3d12::D3D12_RENDER_TARGET_VIEW_DESC>)
    -> Vec<Option<direct3d12::ID3D12Resource>> {
    // bind render target view heap to swap chain buffer
    let mut back_buffers: Vec<Option<direct3d12::ID3D12Resource>> = vec![None; swapchain_desc.buffer_count as usize];

    let handle = unsafe {
                    let mut tmp = direct3d12::D3D12_CPU_DESCRIPTOR_HANDLE::default();
                    descriotor_heap.GetCPUDescriptorHandleForHeapStart(&mut tmp);
                    tmp
                };

    for i in 0..swapchain_desc.buffer_count {
        unsafe {
            swapchain.GetBuffer(
                i as u32,
                &direct3d12::ID3D12Resource::IID,
                back_buffers[i as usize].set_abi()
            )
            .and_some(back_buffers[i as usize].clone());
        };

        unsafe {
            device.CreateRenderTargetView(
                &back_buffers[i as usize],
                p_desc.unwrap_or(ptr::null()),
                handle.clone()
            );
        };

        handle.clone().ptr += unsafe {
            device.GetDescriptorHandleIncrementSize(direct3d12::D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_RTV) as usize
        };
    }

    back_buffers
}

pub fn create_fence(device: &direct3d12::ID3D12Device, initial_value: i32, flags: direct3d12::D3D12_FENCE_FLAGS) -> WinResult<direct3d12::ID3D12Fence> {
    unsafe {
        let mut fence: Option<direct3d12::ID3D12Fence> = None;
        device.
        CreateFence(
            initial_value as u64,
            flags,
            &direct3d12::ID3D12Fence::IID,
            fence.set_abi()
        )
        .and_some(fence)
    }
}

fn create_buffer_map<T>(device: &direct3d12::ID3D12Device, comitted_resource: &CommittedResource, resource: &Vec<T>) -> WinResult<direct3d12::ID3D12Resource> {

    let buffer = unsafe {
        let mut tmp: Option<direct3d12::ID3D12Resource> = None;
        device.
            CreateCommittedResource(
                comitted_resource.p_heap_properties,
                comitted_resource.heap_flags,
                comitted_resource.p_resource_desc,
                comitted_resource.initial_resource_state,
                comitted_resource.p_optimized_clear_value,
                &direct3d12::ID3D12Resource::IID,
                tmp.set_abi()
            )
            .and_some(tmp)
    };

    // buffer map
    let mut buffer_map = std::ptr::null_mut::<Vec<T>>();

    // map buffer to GPU
    unsafe {
        buffer.clone().unwrap().
        Map(0, std::ptr::null_mut(), util::get_pointer_of_interface(&mut buffer_map));
    };
    unsafe {
        buffer_map.copy_from_nonoverlapping(resource.as_ptr().cast::<Vec<T>>(), std::mem::size_of_val(&resource) )
    };
    unsafe {
        buffer.clone().unwrap().
        Unmap(0, std::ptr::null_mut());
    };

    buffer
}

pub fn create_vertex_buffer_resources(
    device: &direct3d12::ID3D12Device,
    comitted_resource: &CommittedResource,
    resource: &Vec<Vertex>)
    -> BufferResources<direct3d12::D3D12_VERTEX_BUFFER_VIEW> {
    let buffer = create_buffer_map(device, comitted_resource, resource);

    let buffer_view = direct3d12::D3D12_VERTEX_BUFFER_VIEW {
        buffer_location : unsafe { buffer.clone().unwrap().GetGPUVirtualAddress() },
        size_in_bytes : (resource.len() * mem::size_of::<Vertex>()) as u32,
        stride_in_bytes : std::mem::size_of_val(&resource[0]) as u32,
    };

    BufferResources {
        buffer_view: buffer_view,
        buffer_object: buffer
    }
}

pub fn create_index_buffer_resources(
    device: &direct3d12::ID3D12Device,
    comitted_resource: &CommittedResource,
    resource: &Vec<u16>)
    -> BufferResources<direct3d12::D3D12_INDEX_BUFFER_VIEW> {
    // reuse vertex buffer desc
    let p_resource_desc = comitted_resource.p_resource_desc as *mut direct3d12::D3D12_RESOURCE_DESC;
    unsafe {
        (*p_resource_desc).width = (std::mem::size_of_val(&resource) * &resource.len()) as u64
    };
    let buffer = create_buffer_map(device, comitted_resource, resource);

    let buffer_view = direct3d12::D3D12_INDEX_BUFFER_VIEW {
        buffer_location : unsafe { buffer.clone().unwrap().GetGPUVirtualAddress() },
        format : dxgi::DXGI_FORMAT::DXGI_FORMAT_R16_UINT,
        size_in_bytes : (resource.len() * mem::size_of::<u16>()) as u32,
    };

    BufferResources {
        buffer_view: buffer_view,
        buffer_object: buffer
    }
}

pub fn create_shader_resource(path: &str, p_entrypoint: &str, p_target: &str) -> WinResult<direct3d11::ID3DBlob> {
    // let include_obj = unsafe {
    //     let include_ptr = ptr::null_mut::<direct3d11::ID3DInclude>();
    //     include_ptr.as_ref().unwrap()
    //         .Open(
    //             direct3d11::D3D_INCLUDE_TYPE::D3D_INCLUDE_LOCAL,
    //             ffi::CString::new(include).unwrap().as_ptr(),
    //             None,
    //             None,
    //             0,
    //         )
    // };

    let mut error_blob: Option<direct3d11::ID3DBlob> = None;

    let shader_blob = unsafe {
        let mut tmp: Option<direct3d11::ID3DBlob> = None;
        direct3d_hlsl::D3DCompileFromFile(
            util::path_to_wide_str(path).as_ptr() as *const u16,
            std::ptr::null_mut(),
            None,
            ffi::CString::new(p_entrypoint).unwrap().as_ptr(),
            ffi::CString::new(p_target).unwrap().as_ptr(),
            direct3d_hlsl::D3DCOMPILE_DEBUG | direct3d_hlsl::D3DCOMPILE_DEBUG,
            0,
            &mut tmp,
            &mut error_blob
        )
        .and_some(tmp)
    };

    // notify compilation result
    match &error_blob {
        None => shader_blob,
        Some(error_blob) => {
            // output compilation error message
            let error_str = unsafe {
                string::String::from_raw_parts(
                error_blob.clone().GetBufferPointer().cast::<u8>(),
                error_blob.clone().GetBufferSize(),
                error_blob.clone().GetBufferSize())
            };
            println!("shader compilation error occured : {}", error_str);

            let result: Result<direct3d11::ID3DBlob, WinError> = Err(
                WinError::new(
                    ErrorCode::E_POINTER,
                    "shader compilation error"
                )
            );

            result
        }
    }
}

pub fn create_root_signature(device: &direct3d12::ID3D12Device) -> WinResult<direct3d12::ID3D12RootSignature> {
    let mut root_signature_desc = direct3d12::D3D12_ROOT_SIGNATURE_DESC::default();
    root_signature_desc.flags = direct3d12::D3D12_ROOT_SIGNATURE_FLAGS::D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT;

    // create root signature binary
    let mut root_signature_blob: Option<direct3d11::ID3DBlob> = None;

    let mut error_blob: Option<direct3d11::ID3DBlob> = None;

    unsafe {
        direct3d12::D3D12SerializeRootSignature(
            &root_signature_desc,
            direct3d12::D3D_ROOT_SIGNATURE_VERSION::D3D_ROOT_SIGNATURE_VERSION_1_0,
            &mut root_signature_blob,
            &mut error_blob
        )
        .and_some(error_blob);
    };

    let mut root_signature: Option<direct3d12::ID3D12RootSignature> = None;

    unsafe {
        device.
            CreateRootSignature(
                0,
                root_signature_blob.clone().unwrap().GetBufferPointer(),
                root_signature_blob.unwrap().GetBufferSize(),
                &direct3d12::ID3D12RootSignature::IID,
                root_signature.set_abi()
            )
            .and_some(root_signature)
    }
}

pub fn create_pipeline_state(device: &direct3d12::ID3D12Device, gr_pipeline: &direct3d12::D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> WinResult<direct3d12::ID3D12PipelineState> {
    unsafe {
        let mut pipeline_state: Option<direct3d12::ID3D12PipelineState> = None;
        device.
            CreateGraphicsPipelineState(
                gr_pipeline,
                &direct3d12::ID3D12PipelineState::IID,
                pipeline_state.set_abi()
            )
            .and_some(pipeline_state)
    }
}

pub fn set_viewport(width: i32, height: i32) -> direct3d12::D3D12_VIEWPORT {
    let mut viewport = direct3d12::D3D12_VIEWPORT::default();
    viewport.width = width as f32;
    viewport.height = height as f32;
    viewport.top_leftx = 0.0;
    viewport.top_lefty = 0.0;
    viewport.max_depth = 1.0;
    viewport.min_depth = 0.0;

    viewport
}

// pub fn set_scissor_rect(width: i32, height: i32) -> direct3d12::D3D12_RECT {
//     let mut scissor_rect = direct3d12::D3D12_RECT::default();
//     scissor_rect.top = 0;
//     scissor_rect.left = 0;
//     scissor_rect.right = scissor_rect.left + width;
//     scissor_rect.bottom = scissor_rect.top + height;

//     scissor_rect
// }
