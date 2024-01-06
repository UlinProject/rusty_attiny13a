
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum SSD1306Cmd {
	MemoryMode = 0x20,
	ColumnAddr = 0x21,
	PageAddr = 0x22,
	SetContrast = 0x81,
	ChargePump = 0x8D,
	SegRemap = 0xA0,
	SegRemap2 = 0xA1, // TODO
	DisplayAllOnResume = 0xA4,
	DisplayAllOn = 0xA5,
	NormalDisplay = 0xA6,
	InvertDisplay = 0xA7,
	SetMultiplex = 0xA8,
	DisplayOff = 0xAE,
	DisplayOn = 0xAF,
	ComScanInc = 0xC0,
	ComScanDec = 0xC8,
	SetDisplayOffset = 0xD3,
	SetDisplayClockDiv = 0xD5,
	SetPrecharge = 0xD9,
	SetCOMpins = 0xDA,
	SetVCOMDetect = 0xDB,

	SetLowColumn = 0x00,
	SetHighColumn = 0x10,
	SetStartLine = 0x40,

	ExternalVCC = 0x01,
	SwitchCapVCC = 0x02,

	RightHorizontalScroll = 0x26,
	LeftHorizontalScroll = 0x27,
	VerticalAndRightHorizontalScroll = 0x29,
	VerticalAndLeftHorizontalScroll = 0x2A,
	DeactivateScroll = 0x2E,
	ActivateScroll = 0x2F,
	SetVerticalScrollArea = 0xA3,
	
	Nop = 0xE3,
}

impl SSD1306Cmd {
	#[inline(always)]
	pub const fn read(self) -> u8 {
		self as _
	}
}