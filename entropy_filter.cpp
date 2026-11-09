#include <iostream>
#include <vector>
#include <cmath>
#include <cstdint>

struct FilterResult {
    bool admit;
    double entropy;
};

double shannon_entropy(const std::vector<uint8_t>& data) {
    if (data.empty()) return 0.0;
    int counts[256] = {0};
    for (uint8_t b : data) counts[b]++;
    double ent = 0.0;
    for (int i = 0; i < 256; i++) {
        if (counts[i] == 0) continue;
        double p = static_cast<double>(counts[i]) / static_cast<double>(data.size());
        ent -= p * std::log2(p);
    }
    return ent;
}

FilterResult admit_packet(const std::vector<uint8_t>& data, double threshold = 1.5) {
    double e = shannon_entropy(data);
    return FilterResult{ e >= threshold, e };
}

int main() {
    std::vector<uint8_t> noise(16, 1);
    std::vector<uint8_t> high = {7, 33, 240, 19, 88, 171, 4, 61, 199, 12, 77, 250, 41, 9, 130, 64};

    auto a = admit_packet(noise);
    auto b = admit_packet(high);

    std::cout << "noise admit=" << a.admit << " entropy=" << a.entropy << "\n";
    std::cout << "high  admit=" << b.admit << " entropy=" << b.entropy << "\n";
    return 0;
}
