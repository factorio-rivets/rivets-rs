--[[
This file automatically generates rust bindings for the defines table.
Run this inside factorio as a /c command.
--]]

local function string_split(inputstr, sep)
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
        return 'str'
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
    else
        error('Unknown type: ' .. type(first))
    end
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
    name = convert_to_valid_rust_identifier(name)

    if is_module then
        local module = ''
        for k, v in pairs(table) do
            module = module .. '\n' .. convert_to_rust(k, v)
        end
        return 'pub mod ' .. name .. ' {\n' ..
        '    use super::*;\n' ..
        do_indent(module) .. '\n}'
    end

    local enum = '#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]\n' ..
        '#[factorio_define(kind = ' .. figure_out_enum_return_type(table) .. ')]\n' ..
        'pub enum ' .. name .. ' {\n'
    for k, representation in pairs(table) do
        if type(representation) == 'string' then
            representation = '"' .. representation .. '"'
        end

        enum = enum .. '    #[value = ' .. representation .. ']\n' ..
            '    ' .. convert_to_valid_rust_identifier(k) .. ',\n'
    end

    return enum .. '}\n'
end

local header = '#![allow(clippy::wildcard_imports)]\n' ..
    '#![allow(clippy::enum_variant_names)]\n' ..
    '#![allow(clippy::too_many_lines)]\n' ..
    '#![allow(clippy::match_same_arms)]\n' ..
    '#![allow(non_camel_case_types)]\n\n' ..
    'pub trait Define<const COUNT: usize>: std::ops::Deref {\n' ..
    '    fn variants() -> &\'static [Self; COUNT] where Self: Sized;\n' ..
    '}\n'

local rust = header .. '\n'
for k, v in pairs(defines) do
    rust = rust .. convert_to_rust(k, v) .. '\n\n'
end

log('\n\nSTARTING POINT\n' .. rust)
