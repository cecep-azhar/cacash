use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hadith {
    pub id: String,
    pub number: u32,
    pub narrator: String,
    pub arabic: String,
    pub translation_id: String,
    pub theme: String,
}

pub fn get_curated_hadiths() -> Vec<Hadith> {
    vec![
        Hadith {
            id: "hadith-01".to_string(),
            number: 1,
            narrator: "HR. Bukhari no. 2072 & Muslim no. 1532".to_string(),
            arabic: "الْبَيِّعَانِ بِالْخِيَارِ مَا لَمْ يَتَفَرَّقَا فَإِنْ صَدَقَا وَبَيَّنَا بُورِكَ لَهُمَا فِي بَيْعِهِمَا وَإِنْ كَتَمَا وَكَذَبَا مُحِقَتْ بَرَكَةُ بَيْعِهِمَا".to_string(),
            translation_id: "Dua orang yang berjual-beli memiliki hak pilih selama belum berpisah. Jika keduanya jujur dan terus terang, maka jual-beli keduanya diberkahi. Namun jika keduanya menyembunyikan cacat dan berdusta, maka keberkahan jual-beli keduanya dimusnahkan.".to_string(),
            theme: "Kejujuran Transaksi".to_string(),
        },
        Hadith {
            id: "hadith-02".to_string(),
            number: 2,
            narrator: "HR. Muslim no. 1002".to_string(),
            arabic: "دِينَارٌ أَنْفَقْتَهُ فِي سَبِيلِ اللَّهِ وَدِينَارٌ أَنْفَقْتَهُ فِي رَقَبَةٍ وَدِينَارٌ تَصَدَّقْتَ بِهِ عَلَى مِسْكِينٍ وَدِينَارٌ أَنْفَقْتَهُ عَلَى أَهْلِكَ أَعْظَمُهَا أَجْرًا الَّذِي أَنْفَقْتَهُ عَلَى أَهْلِكَ".to_string(),
            translation_id: "Satu dinar yang engkau infakkan di jalan Allah, satu dinar untuk memerdekakan budak, satu dinar untuk orang miskin, dan satu dinar untuk nafkah keluargamu; yang paling besar pahalanya adalah yang engkau infakkan untuk keluargamu.".to_string(),
            theme: "Keutamaan Nafkah Keluarga".to_string(),
        },
        Hadith {
            id: "hadith-03".to_string(),
            number: 3,
            narrator: "HR. Bukhari no. 1421 & Muslim no. 1044".to_string(),
            arabic: "الْيَدُ الْعُلْيَا خَيْرٌ مِنَ الْيَدِ السُّفْلَى وَابْدَأْ بِمَنْ تَعُولُ وَخَيْرُ الصَّدَقَةِ عَنْ ظَهْرِ غِنًى".to_string(),
            translation_id: "Tangan yang di atas lebih baik daripada tangan yang di bawah. Dan mulailah dari orang yang menjadi tanggunganmu, dan sebaik-baik sedekah adalah yang dikeluarkan dari kelebihan kebutuhan.".to_string(),
            theme: "Kemandirian & Sedekah".to_string(),
        },
        Hadith {
            id: "hadith-04".to_string(),
            number: 4,
            narrator: "HR. Bukhari no. 2287".to_string(),
            arabic: "مَنْ أَخَذَ أَمْوَالَ النَّاسِ يُرِيدُ أَدَاءَهَا أَدَّى اللَّهُ عَنْهُ وَمَنْ أَخَذَ يُرِيدُ إِتْلاَفَهَا أَتْلَفَهُ اللَّهُ".to_string(),
            translation_id: "Barangsiapa meminjam harta orang lain dengan niat membayarnya, Allah akan memudahkannya melunasinya. Dan barangsiapa meminjam dengan niat merusaknya (tidak bayar), Allah akan membinasakannya.".to_string(),
            theme: "Amanah Hutang".to_string(),
        },
        Hadith {
            id: "hadith-05".to_string(),
            number: 5,
            narrator: "HR. Muslim no. 1598".to_string(),
            arabic: "لَعَنَ رَسُولُ اللَّهِ صَلَّى اللَّهُ عَلَيْهِ وَسَلَّمَ آكِلَ الرِّبَا وَمُؤْكِلَهُ وَكَاتِبَهُ وَشَاهِدَيْهِ وَقَالَ هُمْ سَوَاءٌ".to_string(),
            translation_id: "Rasulullah shallallahu 'alaihi wasallam melaknat pemakan riba, penyetor riba, pencatatnya, dan dua saksinya. Beliau bersabda: 'Mereka itu sama (dalam dosa)'.".to_string(),
            theme: "Bahaya & Larangan Riba".to_string(),
        },
    ]
}

pub fn get_today_hadith() -> Hadith {
    let pool = get_curated_hadiths();
    let day = Utc::now().day0() as usize;
    let idx = day % pool.len();
    pool[idx].clone()
}
