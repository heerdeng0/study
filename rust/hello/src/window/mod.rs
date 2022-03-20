use std::mem::{size_of, zeroed};

#[cfg(windows)]
use winapi::shared::minwindef::{UINT, WPARAM, LPARAM, LRESULT, LPVOID, HINSTANCE, LOWORD};
use winapi::shared::windef::{HWND, HMENU, HBRUSH};
use winapi::um::winnt::{LPCSTR, LPCWSTR};
use winapi::um::winuser::{WNDCLASSEXW, LoadCursorW, LoadIconW, GetMessageW, DispatchMessageW, RegisterClassExW, CreateWindowExW, ShowWindow, MessageBoxA, TranslateMessage, DefWindowProcW, PostQuitMessage}; // functions
use winapi::um::winuser::{IDI_APPLICATION, IDC_ARROW, CS_HREDRAW, CS_VREDRAW, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, WM_DESTROY, SW_SHOWDEFAULT, WM_CREATE, WS_VISIBLE, WS_CHILD, WM_COMMAND}; // const variable
use winapi::um::libloaderapi::GetModuleHandleA;

pub fn main()
{
    //윈도우 제목
    let msg: &str = "rust hello";
    //2바이트 와이드문자열 생성
    let wide: Vec<u16> = to_wstring(msg);

    unsafe
    {
        let h_instance = GetModuleHandleA(0 as LPCSTR); //현재 모듈값구하고
        let wndclass = WNDCLASSEXW //WNDCLASSEXW구조체 내용입력.
        {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            cbClsExtra: 0, //사용x
            cbWndExtra: 0, //사용x
            hbrBackground: 16 as HBRUSH, //하얀색창
            hCursor: LoadCursorW(0 as HINSTANCE, IDC_ARROW), // 커서불러오기
            hIcon: LoadIconW(0 as HINSTANCE, IDI_APPLICATION), //아이콘불러오기
            hIconSm: LoadIconW(0 as HINSTANCE, IDI_APPLICATION), //작은아이콘
            hInstance: h_instance, //모듈값
            lpfnWndProc: Some(wnd_proc), //메세지 콜백함수
            lpszClassName: wide.as_ptr(), //윈도우의 클래스이름
            lpszMenuName: 0 as LPCWSTR, //사용x
            style: CS_HREDRAW | CS_VREDRAW, //창 스타일
        };
        
        //부모 윈도우생성
        CreateWindowExW(
            0,
            wide.as_ptr(), wide.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, 
            0 as HWND, 0 as HMENU, 
            h_instance, 
            0 as *mut winapi::ctypes::c_void
        );
        
        //구조체내용입력 
        match RegisterClassExW(&wndclass)
        {
            0 =>
            {
                //만약 FALSE일시
                MessageBoxA(
                    0 as HWND, 
                    b"Failed to call an RegisterClassEx\0".as_ptr() as *const i8, 
                    b"\0".as_ptr() as *const i8, 
                    0 as UINT
                );
            },
            _atom =>
            {
                //아니라면 그 내용을 _atom에 입력
                let window = CreateWindowExW(
                    0,
                    wide.as_ptr(), wide.as_ptr(),
                    WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
                    0 as HWND, 0 as HMENU,
                    h_instance,
                    0 as LPVOID
                );
                
                if window.is_null() {
                    //만약 nullptr일경우
                    MessageBoxA(
                        0 as HWND,
                        b"failed to load an windows\0".as_ptr() as *const i8,
                        b"\0".as_ptr() as *const i8,
                        0 as UINT
                    );
                } else {
                    //그것도아니라면 창생성
                    ShowWindow(window, SW_SHOWDEFAULT);
                    hide_console_window();
                    let mut msg = zeroed();
                    while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0
                    {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
        }
    };
}

//콘솔창을 숨기는 함수
fn hide_console_window()
{
    //메모리보호를 강요하지않음 (unsafe)
    let window = unsafe
    {
        //콘솔창 핸들값 얻어오기
        kernel32::GetConsoleWindow()
    };
    
    //만약 nullptr이 아닐경우
    if window != std::ptr::null_mut()
    {
        unsafe
        {
            //창 숨기기
            user32::ShowWindow(window, 0 as winapi::ctypes::c_int) 
        };
    }
}

//windows사용명시
#[cfg(windows)]
fn to_wstring(str: &str) -> Vec<u16> //str를 winapi의 wide-string으로 변환하는 함수
{
    use std::ffi::OsStr; //문자열 인코딩관련 라이브러리
    use std::os::windows::ffi::OsStrExt; //같음
    use std::iter::once; //반복자관련 라이브러리
    
    let v: Vec<u16> = OsStr::new(str).encode_wide().chain(once(0).into_iter()).collect(); //인코딩
    return v;
}

#[cfg(windows)] unsafe extern "system"
fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT // __stdcall
{    
    match msg //msg로 패턴매칭
    {
        WM_CREATE => //창생성 메세지
        {
            //윈도우옵션에서 WS_EX_TOPMOST지우기 (메세지박스가 잘안보여서요. 이 옵션은 지웠습니다.)
            winapi::um::winuser::SetWindowLongW(hwnd, winapi::um::winuser::GWL_EXSTYLE, winapi::um::winuser::GetWindowLongW(hwnd, winapi::um::winuser::GWL_EXSTYLE) & !WS_EX_TOPMOST as i32);
            //버튼생성
            CreateWindowExW(0, to_wstring("button").as_ptr(), to_wstring("btn").as_ptr(), WS_VISIBLE | WS_CHILD, 0 as winapi::ctypes::c_int, 0 as winapi::ctypes::c_int, 100 as winapi::ctypes::c_int, 50 as winapi::ctypes::c_int, hwnd, 0x01 as HMENU, 0 as HINSTANCE, 0 as LPVOID);
            0
        },
        WM_COMMAND => //메세지전송 메세지
        {
            let cmd_msg = LOWORD(wparam as u32); //wparam에 저장된 하위비트의 내용을 cmdMsg에 저장
            if cmd_msg == 0x01 //1일시
            {
                //메세지박스출력
                winapi::um::winuser::MessageBoxW(hwnd, to_wstring("message").as_ptr(), to_wstring("title").as_ptr(), winapi::um::winuser::MB_OK);
            }
            0
        },
        WM_DESTROY => //윈도우파괴 메세지
        {
            PostQuitMessage(0);
            0
        },
        _ =>
        {
            return DefWindowProcW(hwnd, msg, wparam, lparam);
        }
    }
}