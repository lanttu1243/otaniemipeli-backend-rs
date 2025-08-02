insert into boards (name) values ('Lauta 2025'), ('Lauta 2001'), ('Lauta 1985');
insert into ingredients (name, abv, carbonated) values        ('Kalja', 4.7, true),
                                                              ('Slime', 40.0, false),
                                                              ('Punaviini', 15.0, false),
                                                              ('Siideri', 5.3, true),
                                                              ('Coca Cola', 0.0, true),
                                                              ('Koskenkorva', 38.0, false),
                                                              ('Tequila', 40.0, false),
                                                              ('Fernet-Branca', 39.0, false),
                                                              ('Thunder', 12.0, false),
                                                              ('Lonkero', 5.5, true),
                                                              ('Vichy', 0.0, true),
                                                              ('A12 Denat', 80.0, false),
                                                              ('Energia Juoma', 0.0, true),
                                                              ('Punssi', 40.0, false),
                                                              ('Jaloviina', 38.0, false),
                                                              ('Appelsiinimehu', 0.0, false),
                                                              ('Jägermeister', 40.0, false),
                                                              ('Minttu', 40.0, false),
                                                              ('Trocadero', 0.0, true),
                                                              ('Absintti', 66.0, false),
                                                              ('Mehukatti', 0.0, false),
                                                              ('Salmari', 32.0, false),
                                                              ('Banaanilikööri', 20.0, false),
                                                              ('Hauki', 20.0, false);
INSERT INTO drinks (name) VALUES ('Kalja'), ('AS-huikka'), ('Slime'),
                              ('Terästetty Kokis'), ('Siideri'),
                              ('Punaviini'), ('Tequila'),
                              ('Fernet-Branca'), ('Thunder'),
                              ('Vichy'), ('Pirtu'), ('Energiajuoma'),
                              ('Kuulalaakeri'), ('Korjaussarja'),
                              ('Vodka'), ('Lammen vesi'), ('Suohauta'),
                              ('Jallu'), ('Trocadero'),
                              ('Terästetty Trocadero'), ('Absintti'),
                              ('Pöhkötaksi'), ('Apoptoosi'),
                              ('Lonkero'), ('Kokis'), ('Varasto'),
                              ('Hauki');
INSERT INTO drink_ingredients (drink_id, ingredient_id, quantity) VALUES
                                (1, 1, 33),
                                (2, 1, 11),
                                (2, 4, 11),
                                (2, 3, 11),
                                (3, 2, 4),
                                (4, 5, 33),
                                (4, 6, 4),
                                (5, 4, 33),
                                (6, 3, 15),
                                (7, 7, 4),
                                (8, 8, 4),
                                (9, 9, 20),
                                (10, 11, 33),
                                (11, 12, 4),
                                (12, 13, 33),
                                (13, 15, 2),
                                (13, 14, 2),
                                (14, 16, 30),
                                (14, 6, 8),
                                (15, 6, 4),
                                (16, 10, 33),
                                (16, 17, 4),
                                (17, 15, 2),
                                (17, 6, 2),
                                (17, 18, 2),
                                (18, 15, 4),
                                (19, 19, 33),
                                (20, 19, 29),
                                (20, 6, 4),
                                (21, 20, 4),
                                (22, 21, 2),
                                (22, 12, 2),
                                (23, 22, 2),
                                (23, 23, 2),
                                (24, 10, 33),
                                (25, 5, 33),
                                (26, 16, 2),
                                (26, 12, 2),
                                (27, 24, 4);

INSERT INTO places (place_name, rule, place_type) VALUES
                                                                       ('Metroasema', 'Kaikki joukkueet aloittavat pelin Tietotien metroaseman luona olevalta pysäkiltä. Jokainen joukkue juo tuomarin päätöksen mukaisen määrän tuomarin päättämää juomaa.', 'special'),
                                                                       ('Normaaliruutu', 'Normaalista ruudusta löydät kaljan. Ruutua ei täytetä', 'normal'),
                                                                       ('A-bloc', 'A Blocissa joukkueelle tarjoillaan HTR:ää (Hyytelön Täyteinen Rasti).', 'food'),
                                                                       ('Raide-Jokeri', 'Raide-Jokeri teleporttaa joukkueen Maarin pysäkille. Matkan aikana nälkä yllättää ja joukkue hankkii K-Marketista lasillisen hyytelöä.', 'special'),
                                                                       ('Oikeustalo', 'Absintti', 'special'),
                                                                       ('Lintutorni', 'Joukkue erottaa tarkoilla korvillaan pöllön huhuilua ja ansaitsee lasin jaloviinaa.', 'special'),
                                                                       ('Suo', 'Lintutornista selvittyään joukkue joutuu vielä kahlaamaan suon läpi ja nauttimaan annoksen suohautaa.', 'special'),
                                                                       ('Alvari', 'Alvariin saapuvaa joukkuetta odottaa tavalliseen tapaan lasillinen hyytelöä.', 'food'),
                                                                       ('Prodekon kiltahuone', 'Prodekon kiltahuoneella juodaan tuopillinen Thunderia. Thunder ei tunnetusti lopu tarpeeksi ajoissa.', 'guild'),
                                                                       ('OLOhuone', 'OLOhuoneella juodaankin tequilaa!', 'guild'),
                                                                       ('Database', 'DG:n kiltahuoneella juomana on glitterillä koristeltua Trocaderoa. Yleisön vaatiessa terästettynä.', 'guild'),
                                                                       ('SIKin kiltahuone', 'SIKin kiltahuoneella tarjoillaan annos kuulalaakeria.', 'guild'),
                                                                       ('ASki', 'AS:n kiltiksellä pelataan AShuikkaa. Siispä ruudussa on keskituoppi sisältäen kaljaa, siideriä ja punkkua suhteessa 1:1:1.', 'guild'),
                                                                       ('TUASin ruokala', 'TUASin ruokalaan saapuvaa joukkuetta odottaa tavalliseen tapaan lasillinen hyytelöä.', 'food'),
                                                                       ('T-talon ruokala', 'T-talon ruokalaan saapuvaa joukkuetta odottaa tavalliseen tapaan lasillinen hyytelöä.', 'food'),
                                                                       ('Design Factory', 'Joukkue keksii uuden startup-idean ja saa suunnitella seuraavalle saapuvalle joukkueelle drinkin isännän ja emännän ilmoittamasta ainesosavalikoimasta.', 'special'),
                                                                       ('Sähkölaitos', 'Sähkölaitokselta saa kaljan sijaan piristävän energiajuoman.', 'special'),
                                                                       ('Vanha kiltis', 'Vanhan kiltahuoneen muistoksi kiltahuoneella on aluksi yksi kalja. Ruutu täydennetään jokaisen joukkueen jälkeen. Jokaisen käynnin jälkeen ruutuun ilmestyy yksi kalja enemmän kuin edellisellä kerralla!', 'guild'),
                                                                       ('IK:n kiltis', 'Rakennusinsinöörikillan kiltahuoneella pelataan pikainen Norske Kimble. Joukkue heittää kahta noppaa ja saa pienemmän silmäluvun mukaisen määrän kaljapulloja juotavakseen.', 'guild'),
                                                                       ('Ossinlampi', 'Joukkue pistäytyy viilentymässä Ossinlammessa ja nauttii lasillisen lammen vettä.', 'special'),
                                                                       ('Kvarkki', 'Joukkue juhlii kvarkkipäivää lasillisella hyytelöä.', 'food'),
                                                                       ('Maarintalo, excubussi', 'Excubussi kaappaa joukuueen mukaansa Tampereelle. Joukkue voi halutessaan tilata bussimatkalle juotavaa (kola, kalja, siideri, punkku).', 'special'),
                                                                       ('Miinusmaa', 'Joukkue saa annoksen haukea.', 'special'),
                                                                       ('Ylioppilaiden terveydenhuoltosäätiö', 'YTHS:llä joukkue saa kaljapullon sijasta juotavakseen snapsilasillisen pirtua. Hätätilassa käy myös esim. A12-denaturoitu sprii.', 'special'),
                                                                       ('Tietokillan varasto', 'Tietokillan varastolla pelataankin ykköstä joukkueiden välillä. Joukkueet heittävät yhtä noppaa alkaen ruutuun saapuneesta joukkueesta. Ensimmäisenä ykkösen heittänyt joukkue voittaa annoksen pelijuomaa, ideaalisesti 1:1 appelsiinimehulla lantrattu 80%:lla vodkalla. Tuplilla ruutuun saapuessa heitetään kahteen ykköseen asti.', 'special'),
                                                                       ('Taikametsä', 'Taikametsään päätyvä joukkue ei voi koskaan tietää mitä saa. Joukkue nauttii noppien silmäluvun osoittaman taikajuoman. Kullekin silmälukuyhdistelmälle kuuluvan juoman päättävät isäntä ja emäntä ennen peliä.', 'special'),
                                                                       ('Kappeli', 'Kappelissa joukkueelle tarjotaan tavanomaisen kaljan sijaan lasillinen ehtoollisviiniä, joka tulee nauttia ennen matkan jatkamista.', 'special'),
                                                                       ('Rantasauna', 'Rantsulta ei kalja lopu!', 'sauna'),
                                                                       ('Gorsu', 'Gorsustahan ei tunnetusti kalja lopu, eli ruutu täytetään jokaisen joukkueen jälkeen.', 'sauna'),
                                                                       ('Smökki', 'Joukkue saapuu Smökkiin sitseille. Tarjolla on tietysti ruokaa ja juomaa eli yksi olut ja yksi hyytelö.', 'food'),
                                                                       ('Ossinsauna', 'Ossinsaunaltakaan ei kalja lopu. Ruutu täytetään jokaisen joukkueen jälkeen.', 'sauna'),
                                                                       ('OK20', 'OK20:n saunoilta ei kalja lopu!', 'sauna'),
                                                                       ('JMT1 aula', 'Törmäät aulassa vanhaan tuttuun, joka tarjoaa Ferraa.', 'special'),
                                                                       ('JMT3', '3A:n saunastakaan ei kalja lopu, eli ruutu täytetään jokaisen joukkueen jälkeen.', 'sauna'),
                                                                       ('Otaranta 8', 'Myöskään Otarannan kattosaunalta ei kalja lopu kesken. Ruutu täytetään jokaisen joukkueen jälkeen.', 'sauna'),
                                                                       ('Täffä', 'Täffällä ei olekaan spagettipäivä vaan hyytelöpäivä.', 'food'),
                                                                       ('Dipoli', 'Dipolissa törmäät rehtoriin, joka kutsuu joukkueen lounaalle. Joukkue saa hyytelöä.', 'food'),
                                                                       ('X-burger', 'X-Burgerilta joukkue hankkii itselleen annoksen hyytelöä.', 'food'),
                                                                       ('Vanha ostari', 'Vanhan ostarin monipuolinen tarjonta innoittaa joukkueen hankkimaan sekä lasillisen hyytelöä että oluen palanpainikkeeksi.', 'food'),
                                                                       ('Aalto-yliopiston Ylioppilaskunta', 'Joukkue eksyy AYY:n keskustoimistolle ja vapaaehtoishommat vievät yllättäen mennessään. Joukkue heittää yhtä noppaa ja siirtyy silmäluvun mukaan taaksepäin. Tuplilla taaksepäin liikutaan tietenkin tuplat.', 'special'),
                                                                       ('Fyssan kiltis', 'Fyysikkokillan kiltiksellä joukkueelle on tarjolla asteen pöhkömpää taksia.', 'guild'),
                                                                       ('Inkubion kiltis', 'Inkubion kiltahuoneella tarjolla on Apoptoosia.', 'guild'),
                                                                       ('Koneen kiltis', 'Koneinsinöörikillan kiltahuoneella joukkue juo tuopin korjaussarjaa.', 'guild'),
                                                                       ('Kiltis', 'Ensimmäisenä Tietokillan kiltahuoneelle saapuva joukkue voittaa pelin!', 'special'),
                                                                       ('Mikontalo', 'Mikontalo tekee kiljua Kioton sopimusta uhmaten! Joukkue juo lasin kotiviiniä eli punkkua.', 'special'),
                                                                       ('Kultainen apina', 'Kapinasta ei kalja lopu. Joukkue juo yhden oluen.', 'special'),
                                                                       ('TiTen kiltahuone', 'Kiltahuone muuttaa Kolaa koodiksi. Joukkue juo lasin kolaa. Terästettynä yleisön vaatiessa.', 'guild'),
                                                                       ('Teekkarisauna', 'Joukkue on päätynyt teekkarisaunalle, jossa ohjelmassa on teekkarisauna™. Joukkue joutuu nauttimaan kolme pulloa kaljaa sekä lasillisen hyytelöä ennen kuin voi jatkaa matkaa.', 'sauna'),
                                                                       ('Tupsula', 'Käyt padassa ja hörppäät puoli litraa patavettä. Varo kortsuja! Ryhmä juo puoli litraa lämmintä lonkeroa, jossa saattaa olla kondomi.', 'special'),
                                                                       ('Etuparkki. excubussi', 'Joukkueen päästessä tähän ruutuun (yli menevät heitot lasketaan) excubussi vie takaisin Otaniemeen. Joukkue voi halutessaan tilata bussimatkalle juotavaa (kola, kalja, siideri, punkku). Otaniemessä joukkue saapuukin sopivasti Smökkiin sitsaamaan.', 'special'),
                                                                        ('Maarinsolmu', 'Joukkue saapuu Maarinsolmuun ja ostaa lasillisen hyytelöä.', 'food'),
                                                                       ('Urheilukenttä', 'Urheilukentän kohdalla sinne ensimmäiseksi saapunut joukkue suorittaa 110 metrin aidat eli juo kolme pulloa olutta, ennenkuin pääsee jatkamaan. Ruutua ei täydennetä!', 'special');
COPY public.board_places (board_id, place_number, place_id, start, "end", x, y) FROM stdin;
1	0	1	t	f	36.00000000000003	89.99999999999997
1	1	2	f	f	40.99999999999998	86.39999999999992
1	2	2	f	f	43.99999999999999	81.00000000000004
1	3	2	f	f	48.19999999999997	83.6
1	35	2	f	f	61.99999999999996	15.600000000000009
1	4	3	f	f	51.20000000000001	88.59999999999998
1	5	2	f	f	55.399999999999984	91.2
1	53	52	f	f	83.80000000000007	66.59999999999998
1	54	36	f	f	77.20000000000006	65.40000000000002
1	55	37	f	f	76.60000000000008	76.60000000000004
1	68	44	f	t	40.2	69.60000000000005
1	69	51	f	f	5.6	82.59999999999995
1	71	5	f	f	11.799999999999995	74.4
1	72	2	f	f	12.6	64.2
1	6	4	f	f	60.2	91
1	7	8	f	f	58.80000000000003	80.59999999999998
1	8	2	f	f	55.399999999999984	76.40000000000003
1	9	2	f	f	51.40000000000001	75.00000000000001
1	73	2	f	f	15.400000000000013	56.99999999999998
1	74	2	f	f	19.999999999999996	51.39999999999998
1	10	2	f	f	47.40000000000001	75.39999999999999
1	11	2	f	f	43.20000000000001	74.79999999999995
1	12	10	f	f	38.80000000000003	75.6
1	13	11	f	f	35.000000000000014	75.39999999999996
1	14	2	f	f	31.4	76.19999999999997
1	15	9	f	f	27.799999999999997	76.59999999999998
1	16	12	f	f	24.200000000000003	75.00000000000001
1	17	13	f	f	27.200000000000003	70.39999999999998
1	18	14	f	f	31.2	70.00000000000003
1	19	15	f	f	36.2	69.6
1	20	2	f	f	39.39999999999999	62.799999999999955
1	21	16	f	f	46.20000000000005	54.59999999999992
1	22	2	f	f	51.20000000000004	49.79999999999994
1	23	17	f	f	57.00000000000001	50.199999999999974
1	24	18	f	f	61.399999999999984	55.000000000000014
1	25	19	f	f	66.59999999999998	55.39999999999996
1	75	6	f	f	24.799999999999994	47
1	76	2	f	f	24.00000000000001	54.79999999999998
1	77	7	f	f	22.00000000000001	60.399999999999864
1	78	2	f	f	21.600000000000005	68.39999999999998
1	79	2	f	f	18.40000000000001	21.399999999999988
1	80	45	f	f	14.40000000000001	20.39999999999999
1	81	2	f	f	13.80000000000001	12.399999999999997
1	82	46	f	f	15.200000000000012	3.3999999999999995
1	83	2	f	f	19.40000000000001	5.3999999999999995
1	26	2	f	f	70.39999999999999	50.80000000000001
1	27	2	f	f	72.8	45.60000000000002
1	28	20	f	f	69.79999999999998	41.800000000000054
1	29	2	f	f	65.99999999999996	41.00000000000007
1	30	21	f	f	62.6	45.20000000000002
1	31	22	f	f	59.00000000000001	39.000000000000014
1	32	23	f	f	57.79999999999999	33.000000000000014
1	33	2	f	f	58.19999999999994	26.39999999999999
1	34	2	f	f	59.399999999999984	19.99999999999999
1	90	2	f	f	35.000000000000036	16.200000000000006
1	91	50	f	f	25.40000000000001	14.800000000000017
1	36	24	f	f	65.2	10.799999999999995
1	37	2	f	f	70.39999999999993	11.999999999999996
1	38	2	f	f	75.19999999999995	12.999999999999996
1	39	25	f	f	78.99999999999989	10.19999999999999
1	40	26	f	f	84.59999999999997	14.800000000000011
1	41	27	f	f	85.19999999999995	21.400000000000023
1	42	2	f	f	90.99999999999996	23.20000000000002
1	43	28	f	f	94.19999999999996	31.399999999999977
1	44	29	f	f	90.4	37.59999999999991
1	45	30	f	f	84.39999999999999	33.399999999999935
1	46	31	f	f	80.39999999999996	35.399999999999906
1	47	2	f	f	78.8	44.2
1	48	32	f	f	77.80000000000004	53.20000000000002
1	49	33	f	f	81.60000000000001	51.39999999999999
1	50	34	f	f	85.80000000000001	46.199999999999946
1	51	35	f	f	89.00000000000003	55.79999999999994
1	52	2	f	f	88.80000000000003	63.19999999999996
1	56	2	f	f	78.80000000000011	84.40000000000003
1	57	38	f	f	76.60000000000002	94.00000000000006
1	58	39	f	f	72.8	90
1	59	40	f	f	70.4	84.4
1	60	2	f	f	67.20000000000003	79.40000000000002
1	61	2	f	f	63.400000000000006	76.20000000000003
1	62	41	f	f	59.79999999999999	67.8
1	63	42	f	f	59.59999999999999	61.79999999999997
1	64	43	f	f	53.99999999999999	59.39999999999999
1	65	2	f	f	50.00000000000005	60.39999999999992
1	66	2	f	f	46.400000000000055	61.79999999999994
1	67	2	f	f	43.399999999999984	66.59999999999998
1	84	47	f	f	21.600000000000012	12.4
1	85	48	f	f	27.40000000000001	6
1	86	2	f	f	33.79999999999996	4.399999999999999
1	87	49	f	f	46.4	3.9999999999999982
1	88	2	f	f	46.200000000000024	20.99999999999999
1	89	2	f	f	41.40000000000004	8.000000000000014
1	70	2	f	f	8.400000000000007	77.39999999999993
\.
COPY public.place_connections (board_id, origin, target, on_land, backwards, dashed) FROM stdin;
1	0	1	f	f	f
1	1	2	f	f	f
1	2	3	f	f	f
1	3	4	f	f	f
1	4	5	f	f	f
1	5	6	f	f	f
1	6	7	f	f	f
1	7	8	f	f	f
1	8	9	f	f	f
1	9	10	f	f	f
1	10	11	f	f	f
1	11	12	f	f	f
1	12	13	f	f	f
1	13	14	f	f	f
1	14	15	f	f	f
1	15	16	f	f	f
1	16	17	f	f	f
1	17	18	f	f	f
1	18	19	f	f	f
1	19	20	f	f	f
1	20	21	f	f	f
1	21	22	f	f	f
1	22	23	f	f	f
1	23	24	f	f	f
1	24	25	f	f	f
1	25	26	f	f	f
1	26	27	f	f	f
1	27	28	f	f	f
1	28	29	f	f	f
1	29	30	f	f	f
1	30	31	f	f	f
1	31	32	f	f	f
1	32	33	f	f	f
1	33	34	f	f	f
1	34	35	f	f	f
1	35	36	f	f	f
1	36	37	f	f	f
1	37	38	f	f	f
1	38	39	f	f	f
1	39	40	f	f	f
1	40	41	f	f	f
1	41	42	f	f	f
1	42	43	f	f	f
1	43	44	f	f	f
1	44	45	f	f	f
1	45	46	f	f	f
1	46	47	f	f	f
1	47	48	f	f	f
1	48	49	f	f	f
1	49	50	f	f	f
1	50	51	f	f	f
1	51	52	f	f	f
1	52	53	f	f	f
1	53	54	f	f	f
1	54	55	f	f	f
1	55	56	f	f	f
1	56	57	f	f	f
1	57	58	f	f	f
1	58	59	f	f	f
1	59	60	f	f	f
1	60	61	f	f	f
1	61	62	f	f	f
1	62	63	f	f	f
1	63	64	f	f	f
1	64	65	f	f	f
1	65	66	f	f	f
1	66	67	f	f	f
1	67	68	f	f	f
1	69	70	f	f	f
1	70	71	f	f	f
1	71	72	f	f	f
1	72	73	f	f	f
1	73	74	f	f	f
1	74	75	f	f	f
1	75	76	f	f	f
1	76	77	f	f	f
1	77	78	f	f	f
1	78	16	f	f	f
1	79	80	f	f	f
1	80	81	f	f	f
1	81	82	f	f	f
1	82	83	f	f	f
1	83	84	f	f	f
1	84	85	f	f	f
1	85	86	f	f	f
1	86	87	f	f	f
1	87	88	f	f	f
1	88	89	f	f	f
1	89	90	f	f	f
1	90	91	f	f	f
1	31	79	t	f	t
1	6	69	t	f	t
1	91	45	t	f	t
\.

COPY public.place_drinks (drink_id, place_number, board_id, refill, optional, n, n_update) FROM stdin;
26	39	1	t	f	1	1
1	1	1	f	f	1	1
1	2	1	f	f	1	1
1	3	1	f	f	1	1
1	5	1	f	f	1	1
1	8	1	f	f	1	1
1	9	1	f	f	1	1
1	10	1	f	f	1	1
1	11	1	f	f	1	1
1	14	1	f	f	1	1
9	15	1	t	f	1	1
13	16	1	t	f	1	1
20	13	1	t	t	1	1
19	13	1	t	t	1	1
7	12	1	t	f	1	1
3	7	1	t	f	1	1
3	4	1	t	f	1	1
2	17	1	t	f	1	1
3	18	1	t	f	1	1
3	19	1	t	f	1	1
1	20	1	f	f	1	1
1	22	1	f	f	1	1
12	23	1	t	f	1	1
1	24	1	t	f	1	N+1
1	25	1	t	f	1	MIN(D2)
1	26	1	f	f	1	1
1	27	1	f	f	1	1
16	28	1	t	f	1	1
1	29	1	f	f	1	1
3	30	1	t	f	1	1
1	31	1	t	t	1	1
25	31	1	t	t	1	1
6	31	1	t	t	1	1
5	31	1	t	t	1	1
27	32	1	t	f	1	1
1	33	1	f	f	1	1
1	34	1	f	f	1	1
1	35	1	f	f	1	1
1	37	1	f	f	1	1
1	38	1	f	f	1	1
11	36	1	t	f	1	1
6	41	1	t	f	1	1
1	42	1	f	f	1	1
1	43	1	t	f	1	1
1	44	1	t	f	1	1
3	45	1	t	f	1	1
1	45	1	t	f	1	1
1	46	1	t	f	1	1
1	47	1	f	f	1	1
1	48	1	t	f	1	1
8	49	1	t	f	1	1
1	50	1	t	f	1	1
1	51	1	t	f	1	1
1	52	1	f	f	1	1
1	53	1	f	f	3	1
3	54	1	t	f	1	1
3	55	1	t	f	1	1
1	56	1	f	f	1	1
1	58	1	t	f	1	1
3	58	1	t	f	1	1
3	57	1	t	f	1	1
1	60	1	f	f	1	1
1	61	1	f	f	1	1
22	62	1	t	f	1	1
23	63	1	t	f	1	1
14	64	1	t	f	1	1
1	65	1	f	f	1	1
1	66	1	f	f	1	1
1	67	1	f	f	1	1
3	69	1	t	f	1	1
1	70	1	f	f	1	1
21	71	1	t	f	1	1
1	72	1	f	f	1	1
1	73	1	f	f	1	1
1	74	1	f	f	1	1
18	75	1	t	f	1	1
1	76	1	f	f	1	1
17	77	1	t	f	1	1
1	78	1	f	f	1	1
1	79	1	f	f	1	1
6	80	1	t	f	1	1
1	81	1	f	f	1	1
1	82	1	t	f	1	1
1	83	1	f	f	1	1
25	84	1	t	t	1	1
4	84	1	t	t	1	1
1	85	1	t	f	3	1
3	85	1	t	f	1	1
1	86	1	f	f	1	1
24	87	1	t	f	1	1
1	88	1	f	f	1	1
1	89	1	f	f	1	1
1	90	1	f	f	1	1
1	91	1	t	t	1	1
25	91	1	t	t	1	1
6	91	1	t	t	1	1
5	91	1	t	t	1	1
\.