use bindings::{
    windows::win32::dxgi as dxgi,
    windows::win32::direct3d12 as direct3d12,
    windows::win32::windows_and_messaging as windows_and_messaging,
    windows::Interface,
    windows::IUnknown,
    windows::ErrorCode,
    windows::win32::direct3d11::D3D_FEATURE_LEVEL,
    windows::Abi,
    windows::Result as WinResult,
};

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

pub fn create_d3d12_device() -> WinResult<direct3d12::ID3D12Device> {
    let levels: [D3D_FEATURE_LEVEL; 4] = [
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_12_0,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0
    ];

    let mut result: ErrorCode = ErrorCode::E_POINTER;
    let mut d3d_device: Option<direct3d12::ID3D12Device> = None;

    for lv in levels.iter() {
        unsafe {
            result = direct3d12::D3D12CreateDevice(
                None,
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
    p_initial_state: &direct3d12::ID3D12PipelineState)
     -> WinResult<direct3d12::ID3D12GraphicsCommandList> {
    unsafe {
        let mut command_list: Option<direct3d12::ID3D12GraphicsCommandList> = None;
        device.
            CreateCommandList(
                node_mask,
                r#type,
                p_command_allocator,
                p_initial_state,
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
