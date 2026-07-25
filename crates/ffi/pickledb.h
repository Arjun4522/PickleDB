#ifndef PICKLEDB_H
#define PICKLEDB_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque database handle */
typedef struct pickledb_t pickledb_t;

/* Result codes */
typedef enum {
    PICKLEDB_OK        =  0,
    PICKLEDB_ERROR     = -1,
    PICKLEDB_NOT_FOUND = -2,
} pickledb_result_t;

/* Open or create a database at the given directory.
 * Returns NULL on failure. */
pickledb_t* pickledb_open(const char* dir);

/* Close the database and free resources. */
void pickledb_close(pickledb_t* db);

/* Insert an encrypted record.
 * data/data_len: pre-encrypted ciphertext (AES-256-GCM)
 * token/token_len: optional 32-byte search token
 * Returns PICKLEDB_OK or PICKLEDB_ERROR. */
pickledb_result_t pickledb_insert(
    pickledb_t* db,
    uint64_t record_id,
    const uint8_t* data,
    size_t data_len,
    const uint8_t* token,
    size_t token_len
);

/* Search for records matching a 32-byte search token.
 * Writes up to *out_count record IDs into out_ids.
 * Returns the total number of matching records, or -1 on error. */
int64_t pickledb_search(
    pickledb_t* db,
    const uint8_t* token,
    size_t token_len,
    uint64_t* out_ids,
    size_t* out_count
);

/* Get an encrypted record by ID.
 * Writes up to *out_data_len bytes into out_data.
 * Returns PICKLEDB_OK, PICKLEDB_NOT_FOUND, or PICKLEDB_ERROR. */
pickledb_result_t pickledb_get(
    pickledb_t* db,
    uint64_t record_id,
    uint8_t* out_data,
    size_t* out_data_len
);

/* Flush all pending writes to durable storage. */
pickledb_result_t pickledb_sync(pickledb_t* db);

#ifdef __cplusplus
}
#endif

#endif /* PICKLEDB_H */
