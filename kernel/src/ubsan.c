// Code from https://gitlab.com/sortix/sortix/blob/master/libc/ubsan/ubsan.c

/*
 * Copyright (c) 2014, 2015 Jonas 'Sortie' Termansen.
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ubsan/ubsan.c
 * Undefined behavior sanitizer runtime support.
 */

#include "common/log.h"
#include <stdint.h>

struct ubsan_source_location {
    const char* filename;
    uint32_t line;
    uint32_t column;
};

struct ubsan_type_descriptor {
    uint16_t type_kind;
    uint16_t type_info;
    char type_name[];
};

typedef uintptr_t ubsan_value_handle_t;

struct ubsan_type_mismatch_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* type;
    uintptr_t alignment;
    unsigned char type_check_kind;
};

void __ubsan_handle_type_mismatch_v1(struct ubsan_type_mismatch_data* data,
                                     ubsan_value_handle_t value) {
    const char* violation = "type mismatch";
    if (!value)
        violation = "type mismatch";
    else if (data->alignment && (value & (data->alignment - 1)))
        violation = "unaligned access";

    log_error("%s @ %s:%u:%u (%x)", violation, data->location.filename,
              data->location.line, data->location.column, value);
}

struct ubsan_overflow_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* type;
};

void __ubsan_handle_add_overflow(struct ubsan_overflow_data* data,
                                 ubsan_value_handle_t lhs,
                                 ubsan_value_handle_t rhs) {
    log_error("addition overflow @ %s:%u:%u (%x %x)", data->location.filename,
              data->location.line, data->location.column, lhs, rhs);
}

void __ubsan_handle_sub_overflow(struct ubsan_overflow_data* data,
                                 ubsan_value_handle_t lhs,
                                 ubsan_value_handle_t rhs) {
    log_error("subtraction overflow @ %s:%u:%u (%x %x)",
              data->location.filename, data->location.line,
              data->location.column, lhs, rhs);
}

void __ubsan_handle_mul_overflow(struct ubsan_overflow_data* data,
                                 ubsan_value_handle_t lhs,
                                 ubsan_value_handle_t rhs) {
    log_error("multiplication overflow @ %s:%u:%u (%x %x)",
              data->location.filename, data->location.line,
              data->location.column, lhs, rhs);
}

void __ubsan_handle_negate_overflow(struct ubsan_overflow_data* data,
                                    ubsan_value_handle_t old_value) {
    log_error("negation overflow @ %s:%u:%u (%x)", data->location.filename,
              data->location.line, data->location.column, old_value);
}

void __ubsan_handle_divrem_overflow(struct ubsan_overflow_data* data,
                                    ubsan_value_handle_t lhs,
                                    ubsan_value_handle_t rhs) {
    log_error("division remainder overflow @ %s:%u:%u (%x %x %s)",
              data->location.filename, data->location.line,
              data->location.column, lhs, rhs, data->type->type_name);
}

struct ubsan_pointer_overflow_data {
    struct ubsan_source_location location;
};

void __ubsan_handle_pointer_overflow(struct ubsan_pointer_overflow_data* data,
                                     ubsan_value_handle_t base,
                                     ubsan_value_handle_t result) {
    log_error("pointer overflow @ %s:%u:%u (%x %x)", data->location.filename,
              data->location.line, data->location.column, base, result);
}

struct ubsan_shift_out_of_bounds_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* lhs_type;
    struct ubsan_type_descriptor* rhs_type;
};

void __ubsan_handle_shift_out_of_bounds(
    struct ubsan_shift_out_of_bounds_data* data, ubsan_value_handle_t lhs,
    ubsan_value_handle_t rhs) {
    log_error("shift out of bounds @ %s:%u:%u (%x %x)", data->location.filename,
              data->location.line, data->location.column, lhs, rhs);
}

struct ubsan_out_of_bounds_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* array_type;
    struct ubsan_type_descriptor* index_type;
};

void __ubsan_handle_out_of_bounds(struct ubsan_out_of_bounds_data* data,
                                  ubsan_value_handle_t index) {
    log_error("out of bounds @ %s:%u:%u (%x)", data->location.filename,
              data->location.line, data->location.column, index);
}

struct ubsan_unreachable_data {
    struct ubsan_source_location location;
};

__attribute__((noreturn)) void
__ubsan_handle_builtin_unreachable(struct ubsan_unreachable_data* data) {
    panic("reached unreachable @ %s:%u:%u", data->location.filename,
          data->location.line, data->location.column);
}

__attribute__((noreturn)) void
__ubsan_handle_missing_return(struct ubsan_unreachable_data* data) {
    panic("missing return @ %s:%u:%u", data->location.filename,
          data->location.line, data->location.column);
}

struct ubsan_invalid_value_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* type;
};

void __ubsan_handle_load_invalid_value(struct ubsan_invalid_value_data* data,
                                       ubsan_value_handle_t value) {
    log_error("invalid value load @ %s:%u:%u (%x)", data->location.filename,
              data->location.line, data->location.column, value);
}

struct ubsan_function_type_mismatch_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* type;
};

void __ubsan_handle_function_type_mismatch(
    struct ubsan_function_type_mismatch_data* data,
    ubsan_value_handle_t value) {
    log_error("function type mismatch @ %s:%u:%u (%x)", data->location.filename,
              data->location.line, data->location.column, value);
}

struct ubsan_nonnull_return_data {
    struct ubsan_source_location location;
    struct ubsan_source_location attr_location;
};

void __ubsan_handle_nonnull_return(struct ubsan_nonnull_return_data* data) {
    log_error("null return @ %s:%u:%u", data->location.filename,
              data->location.line, data->location.column);
}

struct ubsan_nonnull_arg_data {
    struct ubsan_source_location location;
    struct ubsan_source_location attr_location;
};

// TODO: GCC's libubsan does not have the second parameter, but its builtin
//       somehow has it and conflict if we don't match it.
void __ubsan_handle_nonnull_arg(struct ubsan_nonnull_arg_data* data,
                                ubsan_value_handle_t index) {
    log_error("null argument @ %s:%u:%u (%x)", data->location.filename,
              data->location.line, data->location.column, index);
}

struct ubsan_cfi_bad_icall_data {
    struct ubsan_source_location location;
    struct ubsan_type_descriptor* type;
};

void __ubsan_handle_cfi_bad_icall(struct ubsan_cfi_bad_icall_data* data,
                                  ubsan_value_handle_t value) {
    log_error("control flow integrity check failure during indirect call @ "
              "%s:%u:%u (%x)",
              data->location.filename, data->location.line,
              data->location.column, value);
}
