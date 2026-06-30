/* Apollo3 Blue (AMA3B1KK) memory map.
 *   - SRAM at 0x10000000 (not 0x20000000)
 *   - App linked at 0x10000 above the Ambiq Secure Bootloader + SparkFun SVL,
 *     so we flash via svl.py over USB-serial (no SWD probe needed).
 */
MEMORY
{
  FLASH (rx)  : ORIGIN = 0x00010000, LENGTH = 960K - 0x10000
  RAM   (rwx) : ORIGIN = 0x10000000, LENGTH = 384K
}
