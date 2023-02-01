pub(super) fn common_string(index: usize) -> Option<&'static str> {
    match index {
        0 => Some("AABB"),
        5 => Some("AnimationClip"),
        19 => Some("AnimationCurve"),
        34 => Some("AnimationState"),
        49 => Some("Array"),
        55 => Some("Base"),
        60 => Some("BitField"),
        69 => Some("bitset"),
        76 => Some("bool"),
        81 => Some("char"),
        86 => Some("ColorRGBA"),
        96 => Some("Component"),
        106 => Some("data"),
        111 => Some("deque"),
        117 => Some("double"),
        124 => Some("dynamic_array"),
        138 => Some("FastPropertyName"),
        155 => Some("first"),
        161 => Some("float"),
        167 => Some("Font"),
        172 => Some("GameObject"),
        183 => Some("Generic Mono"),
        196 => Some("GradientNEW"),
        208 => Some("GUID"),
        213 => Some("GUIStyle"),
        222 => Some("int"),
        226 => Some("list"),
        231 => Some("long long"),
        241 => Some("map"),
        245 => Some("Matrix4x4f"),
        256 => Some("MdFour"),
        263 => Some("MonoBehaviour"),
        277 => Some("MonoScript"),
        288 => Some("m_ByteSize"),
        299 => Some("m_Curve"),
        307 => Some("m_EditorClassIdentifier"),
        331 => Some("m_EditorHideFlags"),
        349 => Some("m_Enabled"),
        359 => Some("m_ExtensionPtr"),
        374 => Some("m_GameObject"),
        387 => Some("m_Index"),
        395 => Some("m_IsArray"),
        405 => Some("m_IsStatic"),
        416 => Some("m_MetaFlag"),
        427 => Some("m_Name"),
        434 => Some("m_ObjectHideFlags"),
        452 => Some("m_PrefabInternal"),
        469 => Some("m_PrefabParentObject"),
        490 => Some("m_Script"),
        499 => Some("m_StaticEditorFlags"),
        519 => Some("m_Type"),
        526 => Some("m_Version"),
        536 => Some("Object"),
        543 => Some("pair"),
        548 => Some("PPtr<Component>"),
        564 => Some("PPtr<GameObject>"),
        581 => Some("PPtr<Material>"),
        596 => Some("PPtr<MonoBehaviour>"),
        616 => Some("PPtr<MonoScript>"),
        633 => Some("PPtr<Object>"),
        646 => Some("PPtr<Prefab>"),
        659 => Some("PPtr<Sprite>"),
        672 => Some("PPtr<TextAsset>"),
        688 => Some("PPtr<Texture>"),
        702 => Some("PPtr<Texture2D>"),
        718 => Some("PPtr<Transform>"),
        734 => Some("Prefab"),
        741 => Some("Quaternionf"),
        753 => Some("Rectf"),
        759 => Some("RectInt"),
        767 => Some("RectOffset"),
        778 => Some("second"),
        785 => Some("set"),
        789 => Some("short"),
        795 => Some("size"),
        800 => Some("SInt16"),
        807 => Some("SInt32"),
        814 => Some("SInt64"),
        821 => Some("SInt8"),
        827 => Some("staticvector"),
        840 => Some("string"),
        847 => Some("TextAsset"),
        857 => Some("TextMesh"),
        866 => Some("Texture"),
        874 => Some("Texture2D"),
        884 => Some("Transform"),
        894 => Some("TypelessData"),
        907 => Some("UInt16"),
        914 => Some("UInt32"),
        921 => Some("UInt64"),
        928 => Some("UInt8"),
        934 => Some("unsigned int"),
        947 => Some("unsigned long long"),
        966 => Some("unsigned short"),
        981 => Some("vector"),
        988 => Some("Vector2f"),
        997 => Some("Vector3f"),
        1006 => Some("Vector4f"),
        1015 => Some("m_ScriptingClassIdentifier"),
        1042 => Some("Gradient"),
        1051 => Some("Type*"),
        1057 => Some("int2_storage"),
        1070 => Some("int3_storage"),
        1083 => Some("BoundsInt"),
        1093 => Some("m_CorrespondingSourceObject"),
        1121 => Some("m_PrefabInstance"),
        1138 => Some("m_PrefabAsset"),
        1152 => Some("FileSize"),
        1161 => Some("Hash128"),
        _ => None,
    }
}