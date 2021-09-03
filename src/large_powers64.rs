//! Precalculated large powers for 64-bit limbs.

/// Large powers (&[u64]) for base5 operations.
const POW5_1: [u64; 1] = [5];
const POW5_2: [u64; 1] = [25];
const POW5_3: [u64; 1] = [625];
const POW5_4: [u64; 1] = [390625];
const POW5_5: [u64; 1] = [152587890625];
const POW5_6: [u64; 2] = [3273344365508751233, 1262];
const POW5_7: [u64; 3] = [7942358959831785217, 16807427164405733357, 1593091];
const POW5_8: [u64; 5] = [
    279109966635548161,
    2554917779393558781,
    14124656261812188652,
    11976055582626787546,
    2537941837315,
];
const POW5_9: [u64; 10] = [
    13750482914757213185,
    1302999927698857842,
    14936872543252795590,
    2788415840139466767,
    2095640732773017264,
    7205570348933370714,
    7348167152523113408,
    9285516396840364274,
    6907659600622710236,
    349175,
];
const POW5_10: [u64; 19] = [
    8643096425819600897,
    6743743997439985372,
    14059704609098336919,
    10729359125898331411,
    4933048501514368705,
    12258131603170554683,
    2172371001088594721,
    13569903330219142946,
    13809142207969578845,
    16716360519037769646,
    9631256923806107285,
    12866941232305103710,
    1397931361048440292,
    7619627737732970332,
    12725409486282665900,
    11703051443360963910,
    9947078370803086083,
    13966287901448440471,
    121923442132,
];
const POW5_11: [u64; 38] = [
    17679772531488845825,
    2216509366347768155,
    1568689219195129479,
    5511594616325588277,
    1067709417009240089,
    9070650952098657518,
    11515285870634858015,
    2539561553659505564,
    17604889300961091799,
    14511540856854204724,
    12099083339557485471,
    7115240299237943815,
    313979240050606788,
    10004784664717172195,
    15570268847930131473,
    10359715202835930803,
    17685054012115162812,
    13183273382855797757,
    7743260039872919062,
    9284593436392572926,
    11105921222066415013,
    18198799323400703846,
    16314988383739458320,
    4387527177871570570,
    8476708682254672590,
    4925096874831034057,
    14075687868072027455,
    112866656203221926,
    9852830467773230418,
    25755239915196746,
    2201493076310172510,
    8342165458688466438,
    13954006576066379050,
    15193819059903295636,
    12565616718911389531,
    3815854855847885129,
    15696762163583540628,
    805,
];
const POW5_12: [u64; 75] = [
    16359721904723189761,
    5323973632697650495,
    17187956456762001185,
    3930387638628283780,
    3374723710406992273,
    16884225088663222131,
    10967440051041439154,
    9686916182456720060,
    10554548046311730194,
    7390739362393647554,
    6316162333127736719,
    18122464886584070891,
    4044404959645932768,
    3801320885861987401,
    12080950653257274590,
    16414324262488991299,
    16395687498836410113,
    12173633940896186260,
    10843185433142632150,
    11048169832730399808,
    12674828934734683716,
    17370808310130582550,
    10500926985433408692,
    10252725158410704555,
    14170108270502067523,
    3698946465517688080,
    989984870770509463,
    10965601426733943069,
    11389898658438335655,
    6901098232861256586,
    1921335291173932590,
    7662788640922083388,
    9775023833308395430,
    4640401278902814207,
    14532050972198413359,
    8378549018693130223,
    11672322628395371653,
    8930704142764178555,
    6275193859483102017,
    15782593304269205087,
    8673060659034172558,
    8018354414354334043,
    1824896661540749038,
    11345563346725559868,
    14959216444480821949,
    970189517688324683,
    3338835207603007873,
    17684964260791738489,
    1436466329061721851,
    4554134986752476101,
    6398757850768963907,
    4709779218751158342,
    10033277748582410264,
    17932125878679265063,
    10004750887749091440,
    256584531835386932,
    14396282740722731628,
    3086085133731396950,
    17831272085689600064,
    10573926491412564693,
    14888061047859191737,
    4570995450261499817,
    10410165022312935266,
    5691078631447480790,
    8632710455805418155,
    790672778942823293,
    16505464105756800547,
    2092171438149740401,
    17505030673829275878,
    1291290830058928444,
    14856191690683232796,
    8916773426496500052,
    10152003807578858265,
    13104441193763861714,
    649395,
];
const POW5_13: [u64; 149] = [
    15308384451594534913,
    17913664074042735335,
    6115977719198531863,
    5794980608663993169,
    16544350702855106930,
    9253787637781258566,
    4977988951675168190,
    9087837664087448770,
    2098480401110016986,
    15474332540882100712,
    14042133997396540944,
    1090855284423485362,
    12639956485351058381,
    1454115676006639319,
    3180465001342538023,
    14649076551958697729,
    9801292446545910916,
    13552201410826594004,
    6101141927469189381,
    1881431857880609316,
    4907847477899433595,
    8714572486973123228,
    3514969632331374520,
    11667642286891470094,
    2391499697425323350,
    17486585679659076043,
    18267223761882105642,
    2886610765822313148,
    9302834862968900288,
    15246507846733637044,
    15924227519624562840,
    9743741243284697760,
    3159780987244964246,
    7304816812369628428,
    17584602612559717809,
    4146812420657846766,
    14525415362681041515,
    8477630142371600195,
    4380695748062263745,
    12119915994367943173,
    16970630866565485122,
    4332724980155264503,
    8079943140620527639,
    1687908087554405626,
    17051081099834002166,
    12638146269730763230,
    11883749876933445771,
    4662462156371383785,
    4796962238316531176,
    3325504751659868927,
    6469595803187862550,
    5852556621152583005,
    9229334792448387881,
    17979733373938620709,
    13951623534175792756,
    17075879371091039277,
    14212246479457938037,
    4008999959804158260,
    2414266395366403722,
    3252733766253918247,
    6382678985007829216,
    2245927470982310841,
    13790724502051307301,
    13116936866733148041,
    9718402891306794538,
    13516274400356104875,
    17859223875778049403,
    4396895129099725471,
    3563053650368467915,
    12176845952536972668,
    3492050964335269015,
    2740656767075170753,
    4409704077614761919,
    10237775279597492710,
    3314206875098230827,
    16437361028114095448,
    12361736225407656572,
    16792510651790145480,
    11449053143229929935,
    18336641737580333136,
    6558939822118891088,
    4606255756908155300,
    2360792578991605004,
    160428430149144538,
    11644861220729221511,
    10785178451159739786,
    14923560618031934681,
    1902620814992781610,
    14064076995338910412,
    11547019064112212657,
    16847481479966225734,
    8331994491163145469,
    11739712981738851885,
    8008309968651120619,
    10266969595459035264,
    15175153381217702033,
    12208659352573720245,
    7714061140750342961,
    2892831567213510541,
    15453714249045017319,
    71020323573871677,
    15431137995750602633,
    5659146884637671933,
    5998809010488554503,
    16552192379299157850,
    1192197967194298797,
    16157555793424861524,
    10929371590994640255,
    3194469143425738352,
    6651586784672005225,
    11062427140788057791,
    6834443579468668318,
    16421563197797455922,
    6251046422506172884,
    13952303462156793860,
    16632486601871393224,
    11313454360291325172,
    5587835232504462834,
    3105197524618514637,
    18268568531031972989,
    2397205535804309313,
    59413027864729597,
    11869878125348715710,
    12592801707270523266,
    8070632061321113656,
    18403647807860650811,
    267109013517069093,
    6537214311028855260,
    5220826919973709902,
    3448740582779163661,
    16822239213112884941,
    5975299384311048185,
    10294433804430712138,
    4739856055412448774,
    12057273038326387897,
    13119002941950056609,
    3354445304051737058,
    13592813067499314594,
    3890182464434078629,
    17820384357466425060,
    9785228118969879380,
    1778431746734556271,
    10075313876350055029,
    13994048489400919028,
    17948287074199726448,
    2815088342305858722,
    2676626035777198370,
    1174257960026283968,
    421714788677,
];
const POW5_14: [u64; 298] = [
    11471884475673051137,
    8902860357476377573,
    13350296775839230505,
    10609191786344608888,
    7261211985859587338,
    11439672689354862964,
    16789708072300570627,
    4607056528866348430,
    3202978990421512997,
    2024899620433984146,
    17666950207239811774,
    4233228489390288200,
    9137580478688460738,
    4060411066587388546,
    11119949806060600124,
    867715462473090103,
    14382394941384869610,
    4856042377419278489,
    8265605599571137921,
    538981667666252469,
    4270263388700786523,
    3281140600308898503,
    4121392524544394174,
    2077884106245940229,
    9773041957329767574,
    7550623316597646685,
    8611033926449791714,
    18137922955420802793,
    2796546741236224013,
    15477096484628446761,
    9517540128113714010,
    9471917970500821378,
    15938570248662483124,
    5228016831978462619,
    15720991252586974501,
    7662829825220776698,
    17328310068068434348,
    3371736428170309730,
    3803724952191098855,
    13115926536504376719,
    16752571196153442257,
    16540185467776259880,
    3432518182450051120,
    5880364967211798870,
    12355748840305392783,
    14196090758536469575,
    7370123524686686319,
    6819740424617592686,
    13037938013537368753,
    15029273671291927100,
    3671312928327205696,
    7473228676544792780,
    17234079691312938123,
    14164740848093544419,
    13169904779481875902,
    7179036968465894054,
    8244653688947194445,
    17179797746073799490,
    5591970751047577674,
    17530550506268329742,
    5965746721852312330,
    1604149463243472865,
    7734199791463116918,
    11305790396015856714,
    4441196105025505137,
    13046431581185664762,
    124776524294606713,
    1134521334706523966,
    11671728093344476434,
    14103440020972933148,
    3966727403013869059,
    9828094508409132821,
    4355682486381147287,
    10261407143988481234,
    3800455155249557199,
    12700901937937547500,
    18184475466894579360,
    13267691151779895412,
    4714157123477697445,
    10770360171308585263,
    9083344917597998040,
    12078649873810212155,
    18218989082046199377,
    4454285072780637351,
    5287307245618354742,
    16042289702059031730,
    4131926574212754010,
    217692071448455473,
    3624845916216282093,
    2901203491797614218,
    6679177724033967080,
    44561358851332790,
    9094639944041587162,
    13690915012276084311,
    1408896670826320686,
    5359130319612337580,
    6148412925099835601,
    5211368532286409612,
    11386360825549027374,
    16895182466965795071,
    3392940493846427241,
    438089879085393580,
    4783928372776399972,
    6278117363595909959,
    12569481049412674733,
    15648622492570893902,
    1966316336235305115,
    1603775390515993547,
    13576113010204316709,
    10821754650102840474,
    18198222517222903152,
    6966163076615302988,
    1373932372410129684,
    3285839581819684990,
    30177575069719475,
    16447047871247307061,
    11618654126674833808,
    990072222556306872,
    1260682336135768017,
    13862055046689532489,
    15668483092844698432,
    1879572630092764264,
    13912027797058626108,
    6231679788219816920,
    13857858054844167403,
    18101470072534728857,
    4144579812461609229,
    7048589655616599284,
    9946956499532694630,
    9771303850109874038,
    6477823708780339765,
    17526247621747041971,
    13525995675852669549,
    3928768291901239810,
    8094153383078124544,
    11214278667728965552,
    11251547162596832610,
    5964946855123292381,
    3622548288590237903,
    13469765967150053587,
    17798986288523466082,
    14684592818807932259,
    16724077276802963921,
    7119877993753121290,
    1864571304902781632,
    12871984921385213812,
    9065447042604670298,
    3987130777300360550,
    6890545752116901685,
    17275341711601865750,
    6296474927799264658,
    1257436973037243463,
    13854281781965301421,
    1657132483318662716,
    17309399540017292849,
    12808111630089217242,
    1098489625264462071,
    14010458905686364135,
    16134414519481621220,
    14288255900328821475,
    3469093466388187882,
    15982710881468295872,
    4056765540058056052,
    15945176389096104089,
    8625339365793505375,
    12316179968863788913,
    15334123773538054321,
    9536238824220581765,
    16080825720106203271,
    6235695225418121745,
    12035192956458019349,
    3235835166714703698,
    5348960676912581218,
    15315062772709464647,
    17335089708021308662,
    16855855317958414409,
    2369751139431140406,
    3693542588628609043,
    7350405893393987577,
    17402072586341663801,
    7007897690013647122,
    15671767872059304758,
    9259490518292347915,
    14836045474406130394,
    4654005815464502513,
    6487825998330548401,
    7013356660323385022,
    7136200343936679946,
    15341236858676437716,
    3657357368867197449,
    12621075530054608378,
    5603868621997066972,
    7683447656788439942,
    450883379216880060,
    14291494350184945047,
    5466258454997635048,
    14206933098432772126,
    4775870327277641692,
    1864430798867181939,
    13748978265070608793,
    12250822864261576589,
    12561896977498605296,
    16060949594257359328,
    17775189113543311529,
    11835965177892927035,
    4218664174878121437,
    3499000902478111683,
    15169853304359126294,
    7076121963053575143,
    832652347668916805,
    1292148207755194737,
    7556838978364207852,
    5904021986723518500,
    4610244652288570024,
    4526508363195533871,
    746120481022614726,
    737965197247830486,
    4006266184415762653,
    9272188239892688050,
    15346235246415709678,
    11850675997347533184,
    11181059668610842701,
    6687857983250662774,
    2908718488661492818,
    4828337780126983225,
    18071738646453002184,
    12790187227727197880,
    17602483480871623153,
    12523532189621855977,
    10598805712727696716,
    2179787555896149376,
    2242193929457337594,
    14908923241136742532,
    8369182018012550027,
    13385381554043022324,
    3332327430110633913,
    16138090784046208492,
    16172324607469047339,
    8279089815915615244,
    12872906602736235247,
    10894545290539475621,
    15428756545851905023,
    4155747980686992922,
    4074479178894544043,
    66083965608603584,
    13873786284662268377,
    8861183628277687555,
    12119497911296021430,
    2154012318305274287,
    15490706314503067312,
    13643145488710608367,
    672340241093017103,
    6039493278284091973,
    9679797700977436461,
    18070795828318171174,
    2188146431134935377,
    5247392385741514952,
    1852539214842869734,
    12235621681634112739,
    8812930319623534062,
    5585597406294108629,
    11312989214475901864,
    1547377291787797995,
    8641748937186208205,
    12518148659168623694,
    6611379197521520985,
    18096591571068008576,
    15087021227100112139,
    13058454842015958418,
    1473584652966833794,
    4387660670140018168,
    8452836916843525402,
    14376083294443363955,
    13998026203969090659,
    611968444648172645,
    990232438801273845,
    18001186324715561929,
    13470591857250177501,
    14881554140239420091,
    16696367836720124495,
    6328076032778459673,
    17027497695968504616,
    10192245646262428833,
    8282482589527318647,
    4319014353374321425,
    14134087271041670980,
    5060230880114618599,
    13179509240430058600,
    3903514232614801894,
    17774749744702165255,
    15448635507030969726,
    15983775238358480209,
    14542832143965487887,
    9385618098039514666,
    14431419612662304843,
    730863073501675978,
    16750118380379734815,
    9640,
];

#[allow(clippy::redundant_static_lifetimes)]
pub const POW5: [&'static [u64]; 14] = [
    &POW5_1, &POW5_2, &POW5_3, &POW5_4, &POW5_5, &POW5_6, &POW5_7, &POW5_8, &POW5_9, &POW5_10,
    &POW5_11, &POW5_12, &POW5_13, &POW5_14,
];
