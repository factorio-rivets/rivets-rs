function string_split(inputstr, sep)
    if sep == nil then
        sep = '%s'
    end
    local t = {}
    for str in string.gmatch(inputstr, '([^' .. sep .. ']+)') do
        table.insert(t, str)
    end
    return t
end

local function figure_out_enum_return_type(table)
    local _, first = next(table)
    if type(first) == 'string' then
        return '&\'static str'
    elseif type(first) == 'number' then
        local has_negatives = false
        for _, v in pairs(table) do
            if v < 0 then
                has_negatives = true
                break
            end
        end
        local highest_number_of_bits = 0
        for _, v in pairs(table) do
            if has_negatives then
                if v >= -128 and v <= 127 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 8)
                elseif v >= -32768 and v <= 32767 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 16)
                elseif v >= -2147483648 and v <= 2147483647 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 32)
                elseif v >= -9223372036854775808 and v <= 9223372036854775807 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 64)
                else
                    highest_number_of_bits = 128
                    break
                end
            else
                if v <= 255 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 8)
                elseif v <= 65535 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 16)
                elseif v <= 4294967295 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 32)
                elseif v <= 18446744073709551615 then
                    highest_number_of_bits = math.max(highest_number_of_bits, 64)
                else
                    highest_number_of_bits = 128
                    break
                end
            end
        end
        return (has_negatives and 'i' or 'u') .. highest_number_of_bits
    elseif type(first) == 'table' then
        return 'Box<dyn Defines<T>>'
    else
        error('Unknown type: ' .. type(first))
    end
end

local function get_impl(T, name, value_function, key_function, iter_function)
    local impl = 'impl crate::defines::Defines<' .. T .. '> for ' .. name .. ' {\n'
    impl = impl .. value_function .. '        }\n    }\n'
    impl = impl .. key_function .. '        }\n    }\n'
    impl = impl .. iter_function .. '        ]\n    }\n'
    return impl .. '}'
end

local function do_indent(structure)
    return structure:gsub('\n', '\n    ')
end

local function convert_to_valid_rust_identifier(token)
    token = token:gsub('-', '_')
    if token == 'type' then return 'r#type' end
    return token
end

local function convert_to_rust(name, table)
    local _, first = next(table)
    local is_module = type(first) == 'table'
    if is_module then
        local module = ''
        for k, v in pairs(table) do
            module = module .. '\n' .. convert_to_rust(k, v)
        end
        return 'pub mod ' .. name .. ' {' .. do_indent(module) .. '\n}'
    end

    name = convert_to_valid_rust_identifier(name)
    local derives = '#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]\n'
    local enum = derives .. 'pub enum ' .. name .. ' {\n'
    local T = figure_out_enum_return_type(table)
    local value_function = '    fn value(&self) -> ' .. T .. ' {\n' .. '        match self {\n'
    local key_function = '    fn key(&self) -> &\'static str {\n' .. '        match self {\n'
    local iter_function = '    fn iter() -> &\'static [Self] {\n' .. '        &[\n'
    for k, repersentation in pairs(table) do
        enum_key = convert_to_valid_rust_identifier(k)
        if type(repersentation) == 'table' then
            repersentation = name .. '::' .. enum_key
        elseif type(repersentation) == 'string' then
            repersentation = '"' .. repersentation .. '"'
        end
        enum = enum .. '    ' .. enum_key .. ',\n'
        value_function = value_function .. '            Self::' .. enum_key .. ' => ' .. repersentation .. ',\n'
        key_function = key_function .. '            Self::' .. enum_key .. ' => "' .. k .. '",\n'
        iter_function = iter_function .. '            Self::' .. enum_key .. ',\n'
    end
    
    return enum .. '}\n' .. get_impl(T, name, value_function, key_function, iter_function)
end

local function get_defines_trait()
    local trait = 'pub trait Defines<T> {\n'
    trait = trait .. '    fn value(&self) -> T;\n'
    trait = trait .. '    fn key(&self) -> &\'static str;\n'
    trait = trait .. '    fn iter() -> &\'static [Self] where Self: std::marker::Sized;\n'
    trait = trait .. '}\n'
    return trait
end

local attributes = {
    '#![allow(clippy::enum_variant_names)]',
    '#![allow(clippy::too_many_lines)]',
    '#![allow(clippy::match_same_arms)]',
    '#![allow(non_camel_case_types)]',
}

local rust = table.concat(attributes, '\n') .. '\n\n' .. get_defines_trait() .. '\n'
for k, v in pairs(defines) do
    rust = rust .. convert_to_rust(k, v) .. '\n\n'
end

log('\n\nSTARTING POINT\n' .. rust)