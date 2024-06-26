__kernel void fill(__global unsigned char* buffer, unsigned char value) {
    double random = sin((double) value * 0.6578923) * 255;
    if (get_global_id(0) % 4 == 3) {
        buffer[get_global_id(0)] = 255;
    } else {
        if (get_global_id(0) % 4 == 0) {
            buffer[get_global_id(0)] = (unsigned char) random * get_global_id(0);
        } else {
            buffer[get_global_id(0)] = 100;
        }
    }
}