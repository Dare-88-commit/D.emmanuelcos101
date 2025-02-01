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
-- Name: staff staff_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff ALTER COLUMN staff_id SET DEFAULT nextval('public.employees_eid_seq'::regclass);


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
-- Name: employees_eid_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.employees_eid_seq', 11, true);


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
-- PostgreSQL database dump complete
--

