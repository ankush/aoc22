fn main() {
    println!("Max food {}", part1(INPUT));
    println!("Sum of top 3 food {}", part2(INPUT));
}

fn get_elf_foods(inventory: &str) -> Vec<i32> {
    let mut elf_foods = vec![];
    let mut current_food = 0;

    for food in inventory.lines() {
        if food == "" {
            elf_foods.push(current_food);
            current_food = 0;
        } else {
            current_food += food.trim().parse::<i32>().unwrap();
        }
    }
    elf_foods
}

fn part1(inventory: &str) -> i32 {
    let elf_foods = get_elf_foods(inventory);

    elf_foods.iter().max().unwrap().to_owned()
}


fn part2(inventory: &str) -> i32 {
    let mut elf_foods = get_elf_foods(inventory);
    elf_foods.sort();
    elf_foods.reverse();

    let top3_sum = &elf_foods[0..=2];
    top3_sum.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        assert_eq!(part1(""), 0);

        const TEST_INPUT : &str = "1
            41

            40
            1";
        assert_eq!(part1(TEST_INPUT), 42);
    }

    #[test]
    fn part2_works() {
        const TEST_INPUT : &str = "1
            20

            10
            5

            3
            3

            1
            1
            1";
        assert_eq!(part2(TEST_INPUT), 42);
    }
}



const INPUT: &str = "
4456
15332
15148
8795
11382

9808
8430
8486
18918

57935

1604
3015
4529
4862
1822
4297
2568
3263
3011
2127
5186
1947
5816
4255
4041

35216

35747
27324

26368
27055

13138
7084
3464

5645
11098
3052
6917
5235
1717
9228

7789
9290
5753
9600
8631
6873
8844
2228
5123

5177
7591
7973
2093
7266
5307
3562
4994
2681
2924

15474
11021
14987
18512

12187
6865
20315

13751

6467
8287
14768
7194
9595

3948
1577
1989

24800
23570

2378
3426
6782
7276
5242
3067
4899
5532
4877
6314
6935
1956

59761

15344
4376
10193
2616
4785

2213
5782
1524
5941
3889
4619
5429
2136
2728
3576
2656
5164
4329
2497
4708

5683
4966
6373
4478
5416
5769
5070
1858
1402
7860
5079

8074
2328
7620
4129
4601
7599
5920
7630
2371
6012
4981

9498
24782

8021
13701
11586
2602
6357
5024

5196
1576
5951
1324
7173
7755
8917
4876
9529

8424
12572
13981
10565

1250
3812
3169
4603
2583
5161
1338
1190
3905
1306
6034
5014
1395
5712
2405

2979
2522
3492
2056
3938
5299
2344
5770
4986
4396
5009
2297
5204
2212
4639

48407

10536
2349
15857
12566
6685

13146
2316
13286
9084
10813

3229
5725
6745
2572
3680
6120
1520
5282
4233
7415
3651
6321

16325
26001
4135

4446
1777
7851
3725
3467
4269
4832
3126
5262
3832

36676

4235
9133
9303
11635
7756
10581
10205

3881
4503
4561
7864
8425
1132
2225
1774

11546
8235
6565
8843
10925
1294
8524

10308
18232
17418

7868
5254
5010
6247
8073
6653
5905
7280
2294
2310
4373

22935
32902

27527

5453
4894
5399
3194
5313
5029
1675
8505
2402

9019
7439
1022
6928

14254
6993

2241
6280
5528
3050
9740
4417
5394
5005

7238
5994
4777
2224
2615
5435
2882
1197
4046
5816
5169
1982

6078
1024
5313
1077
3849
4504
4905
1531
5982
3421
5897
5863
3924
1462
1498

7730
13740
11124
5254
1477

6460
16501
6967

32786
14401

2100
1443
2242
4612
1688
2989
5955
2558
3471
4266
5055
1696
1675
5607
1698

7442
11417
4391
1381
14939

3197
5254
5713
3793
4821
5224
2793
6292
4170
5175
3118
2921
4915
5183

5820
4391
1572
6231
2637
3245
1151
4649
6362
2701
4472
3981
3061
5232

2014
5908
7546
1550
5985
4555
3104
1570
2728
6966
5707

6900
8301
13106
13291
11480

13286
7865

13378
13699
13111
9125
8693
10880

8246
3428
3820
1395
4372
5028
3319
2791
5672
5250

3849
4034
3471
2861
1981
6597
1121
5171
5684
6322
3404

6530
7349
13396
15417

1544
1116
6318
1411
4235
2121
2289
2782
6496
5601
5820
5679
5084
6093

1188
1305
6501
5374
3059
5159
2258
4739
1082
1300
3228
1862
6513

2311
2736
3212
3840
4430
3264
1314
4005
3845
2290
2132
4929
5428
3038
3314

8080
1334
7997
1600
1403
7378
5766
2404
6620
3227
1285

1453
2076
3420
3210
6937
3434
1551
1308
6724
7854
3566

7944
21317
19792

7309
4116
8109
4976
7840
4063
1899
1282
3849
3471

7881
6265
9472
8410
7233
5480
5932
7062

55574

14717

5572
20925

16502
5704
7737
1666

7731

23304

1843
1296
1902
4620
5012
4071
3017
4109
6039
1343
3176
5562
2259
1205
3274

4160
4008
6960
1483
7779
7941
8578
3089

16288
8081
9553
3391
13430

3134
2771
5632
3394
3806
3416
2820
5295
5551
5993
3275
4223
1026

5349
1116
2873
4787
1400
1427
5019
5063
4101
1009
5664
2278
4839
4351
5563

21339
17904
20186

2835
1127
4130
3382
5698
5093
2716
4206
6223
6229
1566
2929
4368
4734

1534
7504
1597
3334
2415
5265
2596

1768
6531
6030
4418
2215
5698
4344
6017
5690
5953
2643
6509
5571

55789

12897
15223
9267
2635
12196

22537
30299

2392
1543
2911
12733
6014
9408

6161
1422
1149
2998
7005
6205
8151
6756
7996
2657

13763
10436
6205
13561
11215
7667

1175
5134
2837
3469
2453
5981
6131
1145
3177
5624
3275

7324
2404
8478
8607
4730
7408
3153
1243
4409
1557

3562
5873
1366
3030
4244
5379
3185
4384
3227
2918
4867
2392
1259
5097
2502

9159
3112
1546
9707
2973
7748
2498
3408

2754
8701
9383
1032
8853
6561
1620
3548
5350

1573
3449
6148
6428
2047
4891
2734
1906
5358
4574
5609
1203

10709
11865
4776
3416
9062
12053

1445
9294
12245
5102

2250
24101
9067

69003

15391
9789
2491
4529

2296
7458
2953
4324
2693
1774
8020
7327

13589

19833
9336
10146
7848

1860
4327
1136
4033
3876
2293
1458
5180
3298
2402
1055
6011
5404

3232
7872
1774
6694
5737
4408
6366
5436
1659
1404
1906

1901
6314
4317
1004
4615
3045
3354
4869
6198
1700
5942
2906

5559
4176
1851
3937
2463
3997
5604
2202
2277
6054
5874
5895
5024
5604
1052

5333
1443
3930
6574
4939
5709
4451
2005
5770
5812
1904
6412
1455

3836
6852
1460
6465
6736
7842
3138
1588
5173
2141
1280

17550
1366
21148

43430

13946
4212
13110
6545
13440
6113

2153
1069
8788
1013
2072
2615
2721
5305
6362

1967
6352
5793
1378

8623
12864
2992
5445
5729
13215

6387
2940
7403
6638
6829
1085
6876
6992
3060
6603
5952
4335

19705
20102
6637
14071

2840
9088
6723
4085
5522

4121
7644
9835
2363
6791
5901
6526
7273

9599
9676
7347
4769
8184
6267
9319
8997
8320

4400
6279
7783
2220
5400
7160
2884
4431
2137
4215
7009

1771
1273
1351
1636
4824
1078
6062
4117
2073
3731
3259
5938
3854
3518
4303

6145
2840
1092
1074
3794
2474
3662
4088
3412
4375
4870
3283
3987

39487

5939
4462
11269
8957
9176
8451
7384

5844
2701
4494
1323
1355
2817
2844
3357
2690
1485
1292
3127
5413
5130
2611

4375
8058
7707
1222
6437
4629
7025
5385
6516
5607
4211

5857
2485
6192
6899
10078
3717
7144
8567

3500
15273
13240
3718
1469

4015
3971
5568
1927
4210
2791
1790
4671
6421

15712
13378
3252
11034
11821

10668
2660

4227
6926
6629
11649
5860
3738

7839
5273
2817
14372
1373

3174
15890
9917
8555
3689

5067
10140
9829
13507
4641
3012

5103
2025
1916
12487

1759
1966
1463
5374
6313
5308
4942
7268
4051
2697
5120
6179

61363

5018
3780
4918
1108
5395
7109

6220
4245
6322
5329
6383
6852
2839
2890
6796
5522
4376
4184

4395
1770
1627
9959

34941
2163

6620
3240
1464
7213
2440
2416
5710
1413
3111
1238
6759
3243

2891
2268
2816
4543
4885
4940
6805
2388
5045
4455
2311
3000

2389
10749
10902
5409
5879
1523
9673

7299
5735
5399
3058
1453
4879
6755
4358
7490
4373
7078

1091
5077
3905
4431
1314
1057
4299
5526
4051
1816
3438
4255
4814
3497

3154
15076
9075
11062
15034

2602
6838
6717
5069
6021
1197
1433
4996
2150
4477
6854
6793

9470
16177
17085
12206

9346

1537
6246
3425
15612

9179
8255
4268
5929
4779
2064
8454
10415

11847
3833
2302
9676
9392
4772
8144

19068
14793

6674
7489
2800
6290
1561
5104
7672
5826
6534
1625
4518

3782
1168
3350
5836
2756
5196
6134
6217
4238
2673
6462
2703
1792

8058
7336
9255
11102
10372
4544
3400

3560
4786
16166
7505
5275

4106
3782
2824
6291
8768
8953
10093
7212

10343
3707
4931
8986
5122
10412
5659
6535

4446
10953
4583
10386
9446
1290
8175

22060
32800

6529
10560
1565
6744
13510

5644
2683
5983
4928
4287
4229
1238
5676
4459
4428
3596
3357
5361
5864
1651

5500
8352
5151
9023
5639
2401
3444

7207
2938
5187
7597
5537
4185
3302
6283
7436
2060
3284

7910
2995
1029
6938
2543
3021
6932
2344
8607

5999
3983
5017
2519
5665
4522
3997
3826
2525
1787
5833
5077
1046
3403
4119

5644
5785
6978
5278
7358
8732
4155
4893
1033
1915

19270
3726
14601

2839
5182
3385
3874
3909
4896
6049
3513
5858
3585
3914
2003
5358
1410
5380

3094
11126
10227
7659
2491
8200

14804
12807
9658
12943
6693

5524
3874
1823
4654
2071
2935
3342
2845
4694
3711
2366
5002
2433
3191
4709

9068
1009
6619
9325
9608
5953
3186
7929
2219

5762
2410
1474
6659
4007
2793
3423
3281
5319
6674
2826
1156
4955

10835
29983

54869

3054
2100
6896
1702
3616
1500
6799
4683
5443
4554
3588
6136

3632
3894
2685
2075
1191
6239
4758
5575
1776
2966
4951
4317
3836
2731

1141
4772
6072
4153
2855
6357
7153
4671
2976
2115
1080
5259

4357
1328
6412
1730
1320
3812
1910
1794
5786
4504
1435
6854

31138

5461
1426
5065
3397
1932
5237
1533
2796
5136
5350
4613
2637
2148
4779

4256
2636
1308
1811
7268
4693
1371
4370
3211
1140
4636
7428

5699
1987
1999
3069
2646
1763
4447
5638
1443
3903
5526
1362
3249
4218
2294

7625
4988
22798

16793

1861
7119
5038
3479
7553
5253
6487
2741
7276
7550
6363

3075
1057
2116
2172
7377
8013
4129
6875
1448
2830
7384

12623
21242

4377

1769
9780
7910
10742
6711
4762
7717
10483

6545
3475
10066
8506
8200
2227
6908
4278

6071
2457
1180
9584
10016
3239
3736
1508

3503
10764
1139
11624
8885
2515
5928

3981
3009
4024
4701
1220
6493
6906
4025
7476
1317
4816
1378

3632
3672
6583
6796
7802
7499
7110
1681
1680
3025
4679

3318
9634
5843
7936
6508
2891
11554

7264
5595
6633
2288
7357
4230
2282
3014
4755
1351
3039

8150
8979
4245
4293
5803
1759
4522
9330
1655

7650
1117
7239
5993
8370
5845
8114
3561
8626
2534

6934
2534
1559
7600
1738
7202
7935
1518
5011
4768
4957

24403

1528

56664

6240
4652
4254
3303
3071
4044
8469
3620
1773
8207

9158
25624

14332
5930
13744
15194

2402
1443
3448
4152
5758
2976
4905
3556
5756
5650
5321
3509
3411
4310
1001

19554
11935

2065
4464
6093
4054
2308
7369
4077
3870
6820
5414
1793

5148
2885
8779
8411
2368
9738
2555
8994

5852
14172
2551
7236
12618

2422
8142
5695
10672
9666
2944

8975
1384
8737
6814
9050
9875
9607
8206

4843
5431
7512
10125
4505
6737
8943
9244

14044
20015
9074
17692

2065
7381
2923
3124
6842
5312
5653
5376
1565
4015
5192
7431

9860
8742
4134
5648
11254
11136
8152

1039
5878
1432
1569
4399
2468
1825
3572
3745
2358
4034
5380
2482

26567
13363

15462
14772
5927
11524
13281

6209
9734
8595
8729
3465
1289
7405
5511

5986
6743
3343
2098
6385

4039
6421
4520
4805
8409

14688
6273
11459
12152

6351
18681
16311
6119

19159
14867
5035
10867

2712
3110
7319
6181
7037
6428
8742
6499
2804
6266

7577
5035
12874
10877
2675

6449
8890
17826
16514

11281
15231
4709
16501

7356
12005
9773
8760
9469
1131
7310

7410
10398
3919
5813
6592
5149

2045
1223
4611
10603
5080
6080
7489
9028

3065
3881
4241
3426
5614
5759
2629
5616
2336
1553
5978
5694
1634
4055
3091

2782
2657
6272
3793
4696
7070
3286
5580
3374
4220

1716
2651
3943
2643
3944
4430
5825
2297
4633
4508
3840
6493
5414
1304

2784
6732
5215
4494
5889
1661
4418
3013
7460
3200
5443

1541

10562
14833
11492
3651
8946

66600

1064
6728
6738
3987
6073
6542
5077
3811
1996
5854
1099
4302
6179

13444
12866
14105
2630
15546

4802
1420
4264
1558
2066
2357
5290
4610
3583
3210
6250
4217
4974
6001

5902
3243
4171
3138
5095
2204
4890
5105
2199
3912
4942
3711
3037
2622
2873

65623

1861
5546
6520
5393
5139
10765
4894

5894
1786
2167
3468
7533
3517
4168
4394
5193
5610
7018

11160
10085
10310
16158

2341
4501
1022
5132
4226
4486
5816
1641
7403

8201
2009
8548
2442
6605
1095
1629
4854
2912
5726

5987
9926
2171
7810
3620
7589
5873

6421
4638
1520
3227
6149
4707
1261
6029
5076
4185
4973
4604
1872
3370

4377
16755
19423
15389

2947
8703
7654
5575
3505
4490
4992
3683
7613

3015
3581
2208
2834
1582
4397
2506
4176
1902
4690
2965
3352
2660
5352

1415
6549
5701
5793
4212
2582
2429
4059
5696
2582
5499
4773
2401

9861
11375
13345
3901
4954
2597

50834

1759
1506
7576
2350
2044
7635
5237
4473
6065
4242
4038

4753
1707
3708
1955
5728
3405
3049
4492
1785
5396
3591
3046
5805
2310
1145";
