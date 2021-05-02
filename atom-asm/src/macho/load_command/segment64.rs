use atom_macho::load_command::segment64::{
    Section64, SectionAttr, SectionAttrs, SectionType, Segment64,
};

pub fn gen_segment(sections: Vec<Section64>) -> Segment64 {
    let size = sections.iter().map(|sect| sect.size).sum();
    Segment64 {
        cmd_size: Segment64::cmd_size() + Section64::cmd_size() * sections.len() as u32,
        seg_name: "".to_string(),
        vm_addr: 0,
        vm_size: size,
        // あとで書き換える
        file_off: 0,
        file_size: size,
        // ↓↓↓たぶんオブジェクトファイルでは常に7
        max_prot: 7,
        init_prot: 7,
        // ↑↑↑
        flags: 0,
        sections,
    }
}

pub fn gen_text_section() -> Section64 {
    // __TEXT.__text セクションではだいたい
    // ↓の2つのattrを有効にする
    let mut attrs = SectionAttrs::new();
    attrs.push(SectionAttr::SomeInstructions);
    attrs.push(SectionAttr::PureInstructions);

    Section64 {
        sect_name: "__text".to_string(),
        seg_name: "__TEXT".to_string(),
        // ↓↓↓あとで書き換える
        addr: 0,
        size: 0,
        offset: 0,
        align: 0,
        reloff: 0,
        nreloc: 0,
        // ↑↑↑
        flags: (attrs, SectionType::Regular),
        reserved_1: 0,
        reserved_2: 0,
        reserved_3: 0,
    }
}
