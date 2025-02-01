--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character varying(50),
    c_age integer,
    c_email character varying(50),
    c_mobile character varying(15),
    eid integer,
    data_id integer
);


ALTER TABLE public.customer OWNER TO postgres;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10),
    data_duration integer,
    data_price integer
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Name: departments; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.departments (
    dno integer NOT NULL,
    dname character varying(100) NOT NULL,
    location character varying(100) NOT NULL
);


ALTER TABLE public.departments OWNER TO postgres;

--
-- Name: departments_dno_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.departments_dno_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.departments_dno_seq OWNER TO postgres;

--
-- Name: departments_dno_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.departments_dno_seq OWNED BY public.departments.dno;


--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name character varying(100) NOT NULL,
    dno integer NOT NULL,
    staff_sal numeric(10,2) NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Name: employees_eid_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.employees_eid_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.employees_eid_seq OWNER TO postgres;

--
-- Name: employees_eid_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.employees_eid_seq OWNED BY public.staff.staff_id;


--
-- Name: projects; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.projects (
    pno integer NOT NULL,
    pname character varying(100) NOT NULL,
    dno integer NOT NULL,
    duration integer NOT NULL
);


ALTER TABLE public.projects OWNER TO postgres;

--
-- Name: projects_pno_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.projects_pno_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.projects_pno_seq OWNER TO postgres;

--
-- Name: projects_pno_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.projects_pno_seq OWNED BY public.projects.pno;


--
-- Name: departments dno; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.departments ALTER COLUMN dno SET DEFAULT nextval('public.departments_dno_seq'::regclass);


--
-- Name: projects pno; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects ALTER COLUMN pno SET DEFAULT nextval('public.projects_pno_seq'::regclass);


--
-- Name: staff staff_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff ALTER COLUMN staff_id SET DEFAULT nextval('public.employees_eid_seq'::regclass);


--
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiye	43	l_jaiye@gmail.com	08051853411	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	45	p_akonjo@gmail.com	08052356772	100	2
114	Marylene Mapa	31	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055667741	117	11
116	Adams Bree	29	a_bree@gmail.com	08056765424	102	5
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	33	s_adeleke@gmail.com	08067774423	117	11
119	Lawal Tamire	40	l_tamire@gmail.com	09052111101	107	5
120	James Job	28	j_job@gmail.com	08059639119	100	8
121	Matthew Jakande	32	m_jakande@gmail.com	07012324144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.departments (dno, dname, location) FROM stdin;
1	HR	Lagos
2	Finance	Abuja
3	IT	Port Harcourt
4	Marketing	Ibadan
5	Operations	Kano
6	Customer Support	Enugu
7	Engineering	Jos
\.


--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.projects (pno, pname, dno, duration) FROM stdin;
1	ERP System	3	12
2	Customer Portal	6	8
3	Marketing Campaign	4	6
4	Network Expansion	5	24
5	HR Software	1	10
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
2	Mustapha Ali	3	175000.00	32	08063285831
3	Alokwe Martin	7	380000.00	48	07090082812
5	Dankade Aminat	5	550000.00	40	09023688832
6	Josiah Joshua	1	120000.00	30	08053189131
7	Mankinde Mary	2	450000.00	55	09023487830
8	Adeleke Jane	4	200000.00	38	07061045862
9	Osahon Mark	6	320000.00	44	08022289842
10	Suleman Ajayi	3	800000.00	50	07030089811
11	Kuti Lawal	1	750000.00	35	09145689842
\.


--
-- Name: departments_dno_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.departments_dno_seq', 7, true);


--
-- Name: employees_eid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.employees_eid_seq', 11, true);


--
-- Name: projects_pno_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.projects_pno_seq', 5, true);


--
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (dno);


--
-- Name: staff employees_phone_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_phone_key UNIQUE (mobile);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (pno);


--
-- Name: projects projects_dno_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_dno_fkey FOREIGN KEY (dno) REFERENCES public.departments(dno);


--
-- PostgreSQL database dump complete
--

