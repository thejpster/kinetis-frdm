// *****************************************************************************
//
// lm4f120h5qr.h - LM4F120H5QR Register Definitions
//
// Copyright (c) 2011-2012 Texas Instruments Incorporated.  All rights reserved.
// Software License Agreement
//
//   Redistribution and use in source and binary forms, with or without
//   modification, are permitted provided that the following conditions
//   are met:
//
//   Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
//   Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in the
//   documentation and/or other materials provided with the
//   distribution.
//
//   Neither the name of Texas Instruments Incorporated nor the names of
//   its contributors may be used to endorse or promote products derived
//   from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This is part of revision 9453 of the Stellaris Firmware Development Package.
//
// Ported from C to Rust by theJPster <github@thejpster.org.uk>
//
// *****************************************************************************

#![allow(dead_code)]

// *****************************************************************************
//
// Watchdog Timer registers (WATCHDOG0)
//
// *****************************************************************************
macro_rules! WATCHDOG0_LOAD_R        {() => (0x40000000 as *mut u32);}
macro_rules! WATCHDOG0_VALUE_R       {() => (0x40000004 as *mut u32);}
macro_rules! WATCHDOG0_CTL_R         {() => (0x40000008 as *mut u32);}
macro_rules! WATCHDOG0_ICR_R         {() => (0x4000000C as *mut u32);}
macro_rules! WATCHDOG0_RIS_R         {() => (0x40000010 as *mut u32);}
macro_rules! WATCHDOG0_MIS_R         {() => (0x40000014 as *mut u32);}
macro_rules! WATCHDOG0_TEST_R        {() => (0x40000418 as *mut u32);}
macro_rules! WATCHDOG0_LOCK_R        {() => (0x40000C00 as *mut u32);}

// *****************************************************************************
//
// Watchdog Timer registers (WATCHDOG1)
//
// *****************************************************************************
macro_rules! WATCHDOG1_LOAD_R        {() => (0x40001000 as *mut u32);}
macro_rules! WATCHDOG1_VALUE_R       {() => (0x40001004 as *mut u32);}
macro_rules! WATCHDOG1_CTL_R         {() => (0x40001008 as *mut u32);}
macro_rules! WATCHDOG1_ICR_R         {() => (0x4000100C as *mut u32);}
macro_rules! WATCHDOG1_RIS_R         {() => (0x40001010 as *mut u32);}
macro_rules! WATCHDOG1_MIS_R         {() => (0x40001014 as *mut u32);}
macro_rules! WATCHDOG1_TEST_R        {() => (0x40001418 as *mut u32);}
macro_rules! WATCHDOG1_LOCK_R        {() => (0x40001C00 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTA)
//
// *****************************************************************************
macro_rules! GPIO_PORTA_DATA_BITS_R  {() => (0x40004000 as *mut u32);}
macro_rules! GPIO_PORTA_DATA_R       {() => (0x400043FC as *mut u32);}
macro_rules! GPIO_PORTA_DIR_R        {() => (0x40004400 as *mut u32);}
macro_rules! GPIO_PORTA_IS_R         {() => (0x40004404 as *mut u32);}
macro_rules! GPIO_PORTA_IBE_R        {() => (0x40004408 as *mut u32);}
macro_rules! GPIO_PORTA_IEV_R        {() => (0x4000440C as *mut u32);}
macro_rules! GPIO_PORTA_IM_R         {() => (0x40004410 as *mut u32);}
macro_rules! GPIO_PORTA_RIS_R        {() => (0x40004414 as *mut u32);}
macro_rules! GPIO_PORTA_MIS_R        {() => (0x40004418 as *mut u32);}
macro_rules! GPIO_PORTA_ICR_R        {() => (0x4000441C as *mut u32);}
macro_rules! GPIO_PORTA_AFSEL_R      {() => (0x40004420 as *mut u32);}
macro_rules! GPIO_PORTA_DR2R_R       {() => (0x40004500 as *mut u32);}
macro_rules! GPIO_PORTA_DR4R_R       {() => (0x40004504 as *mut u32);}
macro_rules! GPIO_PORTA_DR8R_R       {() => (0x40004508 as *mut u32);}
macro_rules! GPIO_PORTA_ODR_R        {() => (0x4000450C as *mut u32);}
macro_rules! GPIO_PORTA_PUR_R        {() => (0x40004510 as *mut u32);}
macro_rules! GPIO_PORTA_PDR_R        {() => (0x40004514 as *mut u32);}
macro_rules! GPIO_PORTA_SLR_R        {() => (0x40004518 as *mut u32);}
macro_rules! GPIO_PORTA_DEN_R        {() => (0x4000451C as *mut u32);}
macro_rules! GPIO_PORTA_LOCK_R       {() => (0x40004520 as *mut u32);}
macro_rules! GPIO_PORTA_CR_R         {() => (0x40004524 as *mut u32);}
macro_rules! GPIO_PORTA_AMSEL_R      {() => (0x40004528 as *mut u32);}
macro_rules! GPIO_PORTA_PCTL_R       {() => (0x4000452C as *mut u32);}
macro_rules! GPIO_PORTA_ADCCTL_R     {() => (0x40004530 as *mut u32);}
macro_rules! GPIO_PORTA_DMACTL_R     {() => (0x40004534 as *mut u32);}
macro_rules! GPIO_PORTA_SI_R         {() => (0x40004538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTB)
//
// *****************************************************************************
macro_rules! GPIO_PORTB_DATA_BITS_R  {() => (0x40005000 as *mut u32);}
macro_rules! GPIO_PORTB_DATA_R       {() => (0x400053FC as *mut u32);}
macro_rules! GPIO_PORTB_DIR_R        {() => (0x40005400 as *mut u32);}
macro_rules! GPIO_PORTB_IS_R         {() => (0x40005404 as *mut u32);}
macro_rules! GPIO_PORTB_IBE_R        {() => (0x40005408 as *mut u32);}
macro_rules! GPIO_PORTB_IEV_R        {() => (0x4000540C as *mut u32);}
macro_rules! GPIO_PORTB_IM_R         {() => (0x40005410 as *mut u32);}
macro_rules! GPIO_PORTB_RIS_R        {() => (0x40005414 as *mut u32);}
macro_rules! GPIO_PORTB_MIS_R        {() => (0x40005418 as *mut u32);}
macro_rules! GPIO_PORTB_ICR_R        {() => (0x4000541C as *mut u32);}
macro_rules! GPIO_PORTB_AFSEL_R      {() => (0x40005420 as *mut u32);}
macro_rules! GPIO_PORTB_DR2R_R       {() => (0x40005500 as *mut u32);}
macro_rules! GPIO_PORTB_DR4R_R       {() => (0x40005504 as *mut u32);}
macro_rules! GPIO_PORTB_DR8R_R       {() => (0x40005508 as *mut u32);}
macro_rules! GPIO_PORTB_ODR_R        {() => (0x4000550C as *mut u32);}
macro_rules! GPIO_PORTB_PUR_R        {() => (0x40005510 as *mut u32);}
macro_rules! GPIO_PORTB_PDR_R        {() => (0x40005514 as *mut u32);}
macro_rules! GPIO_PORTB_SLR_R        {() => (0x40005518 as *mut u32);}
macro_rules! GPIO_PORTB_DEN_R        {() => (0x4000551C as *mut u32);}
macro_rules! GPIO_PORTB_LOCK_R       {() => (0x40005520 as *mut u32);}
macro_rules! GPIO_PORTB_CR_R         {() => (0x40005524 as *mut u32);}
macro_rules! GPIO_PORTB_AMSEL_R      {() => (0x40005528 as *mut u32);}
macro_rules! GPIO_PORTB_PCTL_R       {() => (0x4000552C as *mut u32);}
macro_rules! GPIO_PORTB_ADCCTL_R     {() => (0x40005530 as *mut u32);}
macro_rules! GPIO_PORTB_DMACTL_R     {() => (0x40005534 as *mut u32);}
macro_rules! GPIO_PORTB_SI_R         {() => (0x40005538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTC)
//
// *****************************************************************************
macro_rules! GPIO_PORTC_DATA_BITS_R  {() => (0x40006000 as *mut u32);}
macro_rules! GPIO_PORTC_DATA_R       {() => (0x400063FC as *mut u32);}
macro_rules! GPIO_PORTC_DIR_R        {() => (0x40006400 as *mut u32);}
macro_rules! GPIO_PORTC_IS_R         {() => (0x40006404 as *mut u32);}
macro_rules! GPIO_PORTC_IBE_R        {() => (0x40006408 as *mut u32);}
macro_rules! GPIO_PORTC_IEV_R        {() => (0x4000640C as *mut u32);}
macro_rules! GPIO_PORTC_IM_R         {() => (0x40006410 as *mut u32);}
macro_rules! GPIO_PORTC_RIS_R        {() => (0x40006414 as *mut u32);}
macro_rules! GPIO_PORTC_MIS_R        {() => (0x40006418 as *mut u32);}
macro_rules! GPIO_PORTC_ICR_R        {() => (0x4000641C as *mut u32);}
macro_rules! GPIO_PORTC_AFSEL_R      {() => (0x40006420 as *mut u32);}
macro_rules! GPIO_PORTC_DR2R_R       {() => (0x40006500 as *mut u32);}
macro_rules! GPIO_PORTC_DR4R_R       {() => (0x40006504 as *mut u32);}
macro_rules! GPIO_PORTC_DR8R_R       {() => (0x40006508 as *mut u32);}
macro_rules! GPIO_PORTC_ODR_R        {() => (0x4000650C as *mut u32);}
macro_rules! GPIO_PORTC_PUR_R        {() => (0x40006510 as *mut u32);}
macro_rules! GPIO_PORTC_PDR_R        {() => (0x40006514 as *mut u32);}
macro_rules! GPIO_PORTC_SLR_R        {() => (0x40006518 as *mut u32);}
macro_rules! GPIO_PORTC_DEN_R        {() => (0x4000651C as *mut u32);}
macro_rules! GPIO_PORTC_LOCK_R       {() => (0x40006520 as *mut u32);}
macro_rules! GPIO_PORTC_CR_R         {() => (0x40006524 as *mut u32);}
macro_rules! GPIO_PORTC_AMSEL_R      {() => (0x40006528 as *mut u32);}
macro_rules! GPIO_PORTC_PCTL_R       {() => (0x4000652C as *mut u32);}
macro_rules! GPIO_PORTC_ADCCTL_R     {() => (0x40006530 as *mut u32);}
macro_rules! GPIO_PORTC_DMACTL_R     {() => (0x40006534 as *mut u32);}
macro_rules! GPIO_PORTC_SI_R         {() => (0x40006538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTD)
//
// *****************************************************************************
macro_rules! GPIO_PORTD_DATA_BITS_R  {() => (0x40007000 as *mut u32);}
macro_rules! GPIO_PORTD_DATA_R       {() => (0x400073FC as *mut u32);}
macro_rules! GPIO_PORTD_DIR_R        {() => (0x40007400 as *mut u32);}
macro_rules! GPIO_PORTD_IS_R         {() => (0x40007404 as *mut u32);}
macro_rules! GPIO_PORTD_IBE_R        {() => (0x40007408 as *mut u32);}
macro_rules! GPIO_PORTD_IEV_R        {() => (0x4000740C as *mut u32);}
macro_rules! GPIO_PORTD_IM_R         {() => (0x40007410 as *mut u32);}
macro_rules! GPIO_PORTD_RIS_R        {() => (0x40007414 as *mut u32);}
macro_rules! GPIO_PORTD_MIS_R        {() => (0x40007418 as *mut u32);}
macro_rules! GPIO_PORTD_ICR_R        {() => (0x4000741C as *mut u32);}
macro_rules! GPIO_PORTD_AFSEL_R      {() => (0x40007420 as *mut u32);}
macro_rules! GPIO_PORTD_DR2R_R       {() => (0x40007500 as *mut u32);}
macro_rules! GPIO_PORTD_DR4R_R       {() => (0x40007504 as *mut u32);}
macro_rules! GPIO_PORTD_DR8R_R       {() => (0x40007508 as *mut u32);}
macro_rules! GPIO_PORTD_ODR_R        {() => (0x4000750C as *mut u32);}
macro_rules! GPIO_PORTD_PUR_R        {() => (0x40007510 as *mut u32);}
macro_rules! GPIO_PORTD_PDR_R        {() => (0x40007514 as *mut u32);}
macro_rules! GPIO_PORTD_SLR_R        {() => (0x40007518 as *mut u32);}
macro_rules! GPIO_PORTD_DEN_R        {() => (0x4000751C as *mut u32);}
macro_rules! GPIO_PORTD_LOCK_R       {() => (0x40007520 as *mut u32);}
macro_rules! GPIO_PORTD_CR_R         {() => (0x40007524 as *mut u32);}
macro_rules! GPIO_PORTD_AMSEL_R      {() => (0x40007528 as *mut u32);}
macro_rules! GPIO_PORTD_PCTL_R       {() => (0x4000752C as *mut u32);}
macro_rules! GPIO_PORTD_ADCCTL_R     {() => (0x40007530 as *mut u32);}
macro_rules! GPIO_PORTD_DMACTL_R     {() => (0x40007534 as *mut u32);}
macro_rules! GPIO_PORTD_SI_R         {() => (0x40007538 as *mut u32);}

// *****************************************************************************
//
// SSI registers (SSI0)
//
// *****************************************************************************
macro_rules! SSI0_CR0_R              {() => (0x40008000 as *mut u32);}
macro_rules! SSI0_CR1_R              {() => (0x40008004 as *mut u32);}
macro_rules! SSI0_DR_R               {() => (0x40008008 as *mut u32);}
macro_rules! SSI0_SR_R               {() => (0x4000800C as *mut u32);}
macro_rules! SSI0_CPSR_R             {() => (0x40008010 as *mut u32);}
macro_rules! SSI0_IM_R               {() => (0x40008014 as *mut u32);}
macro_rules! SSI0_RIS_R              {() => (0x40008018 as *mut u32);}
macro_rules! SSI0_MIS_R              {() => (0x4000801C as *mut u32);}
macro_rules! SSI0_ICR_R              {() => (0x40008020 as *mut u32);}
macro_rules! SSI0_DMACTL_R           {() => (0x40008024 as *mut u32);}
macro_rules! SSI0_CC_R               {() => (0x40008FC8 as *mut u32);}

// *****************************************************************************
//
// SSI registers (SSI1)
//
// *****************************************************************************
macro_rules! SSI1_CR0_R              {() => (0x40009000 as *mut u32);}
macro_rules! SSI1_CR1_R              {() => (0x40009004 as *mut u32);}
macro_rules! SSI1_DR_R               {() => (0x40009008 as *mut u32);}
macro_rules! SSI1_SR_R               {() => (0x4000900C as *mut u32);}
macro_rules! SSI1_CPSR_R             {() => (0x40009010 as *mut u32);}
macro_rules! SSI1_IM_R               {() => (0x40009014 as *mut u32);}
macro_rules! SSI1_RIS_R              {() => (0x40009018 as *mut u32);}
macro_rules! SSI1_MIS_R              {() => (0x4000901C as *mut u32);}
macro_rules! SSI1_ICR_R              {() => (0x40009020 as *mut u32);}
macro_rules! SSI1_DMACTL_R           {() => (0x40009024 as *mut u32);}
macro_rules! SSI1_CC_R               {() => (0x40009FC8 as *mut u32);}

// *****************************************************************************
//
// SSI registers (SSI2)
//
// *****************************************************************************
macro_rules! SSI2_CR0_R              {() => (0x4000A000 as *mut u32);}
macro_rules! SSI2_CR1_R              {() => (0x4000A004 as *mut u32);}
macro_rules! SSI2_DR_R               {() => (0x4000A008 as *mut u32);}
macro_rules! SSI2_SR_R               {() => (0x4000A00C as *mut u32);}
macro_rules! SSI2_CPSR_R             {() => (0x4000A010 as *mut u32);}
macro_rules! SSI2_IM_R               {() => (0x4000A014 as *mut u32);}
macro_rules! SSI2_RIS_R              {() => (0x4000A018 as *mut u32);}
macro_rules! SSI2_MIS_R              {() => (0x4000A01C as *mut u32);}
macro_rules! SSI2_ICR_R              {() => (0x4000A020 as *mut u32);}
macro_rules! SSI2_DMACTL_R           {() => (0x4000A024 as *mut u32);}
macro_rules! SSI2_CC_R               {() => (0x4000AFC8 as *mut u32);}

// *****************************************************************************
//
// SSI registers (SSI3)
//
// *****************************************************************************
macro_rules! SSI3_CR0_R              {() => (0x4000B000 as *mut u32);}
macro_rules! SSI3_CR1_R              {() => (0x4000B004 as *mut u32);}
macro_rules! SSI3_DR_R               {() => (0x4000B008 as *mut u32);}
macro_rules! SSI3_SR_R               {() => (0x4000B00C as *mut u32);}
macro_rules! SSI3_CPSR_R             {() => (0x4000B010 as *mut u32);}
macro_rules! SSI3_IM_R               {() => (0x4000B014 as *mut u32);}
macro_rules! SSI3_RIS_R              {() => (0x4000B018 as *mut u32);}
macro_rules! SSI3_MIS_R              {() => (0x4000B01C as *mut u32);}
macro_rules! SSI3_ICR_R              {() => (0x4000B020 as *mut u32);}
macro_rules! SSI3_DMACTL_R           {() => (0x4000B024 as *mut u32);}
macro_rules! SSI3_CC_R               {() => (0x4000BFC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART0)
//
// *****************************************************************************
macro_rules! UART0_DR_R              {() => (0x4000C000 as *mut u32);}
macro_rules! UART0_RSR_R             {() => (0x4000C004 as *mut u32);}
macro_rules! UART0_ECR_R             {() => (0x4000C004 as *mut u32);}
macro_rules! UART0_FR_R              {() => (0x4000C018 as *mut u32);}
macro_rules! UART0_ILPR_R            {() => (0x4000C020 as *mut u32);}
macro_rules! UART0_IBRD_R            {() => (0x4000C024 as *mut u32);}
macro_rules! UART0_FBRD_R            {() => (0x4000C028 as *mut u32);}
macro_rules! UART0_LCRH_R            {() => (0x4000C02C as *mut u32);}
macro_rules! UART0_CTL_R             {() => (0x4000C030 as *mut u32);}
macro_rules! UART0_IFLS_R            {() => (0x4000C034 as *mut u32);}
macro_rules! UART0_IM_R              {() => (0x4000C038 as *mut u32);}
macro_rules! UART0_RIS_R             {() => (0x4000C03C as *mut u32);}
macro_rules! UART0_MIS_R             {() => (0x4000C040 as *mut u32);}
macro_rules! UART0_ICR_R             {() => (0x4000C044 as *mut u32);}
macro_rules! UART0_DMACTL_R          {() => (0x4000C048 as *mut u32);}
macro_rules! UART0_LCTL_R            {() => (0x4000C090 as *mut u32);}
macro_rules! UART0_LSS_R             {() => (0x4000C094 as *mut u32);}
macro_rules! UART0_LTIM_R            {() => (0x4000C098 as *mut u32);}
macro_rules! UART0_9BITADDR_R        {() => (0x4000C0A4 as *mut u32);}
macro_rules! UART0_9BITAMASK_R       {() => (0x4000C0A8 as *mut u32);}
macro_rules! UART0_PP_R              {() => (0x4000CFC0 as *mut u32);}
macro_rules! UART0_CC_R              {() => (0x4000CFC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART1)
//
// *****************************************************************************
macro_rules! UART1_DR_R              {() => (0x4000D000 as *mut u32);}
macro_rules! UART1_RSR_R             {() => (0x4000D004 as *mut u32);}
macro_rules! UART1_ECR_R             {() => (0x4000D004 as *mut u32);}
macro_rules! UART1_FR_R              {() => (0x4000D018 as *mut u32);}
macro_rules! UART1_ILPR_R            {() => (0x4000D020 as *mut u32);}
macro_rules! UART1_IBRD_R            {() => (0x4000D024 as *mut u32);}
macro_rules! UART1_FBRD_R            {() => (0x4000D028 as *mut u32);}
macro_rules! UART1_LCRH_R            {() => (0x4000D02C as *mut u32);}
macro_rules! UART1_CTL_R             {() => (0x4000D030 as *mut u32);}
macro_rules! UART1_IFLS_R            {() => (0x4000D034 as *mut u32);}
macro_rules! UART1_IM_R              {() => (0x4000D038 as *mut u32);}
macro_rules! UART1_RIS_R             {() => (0x4000D03C as *mut u32);}
macro_rules! UART1_MIS_R             {() => (0x4000D040 as *mut u32);}
macro_rules! UART1_ICR_R             {() => (0x4000D044 as *mut u32);}
macro_rules! UART1_DMACTL_R          {() => (0x4000D048 as *mut u32);}
macro_rules! UART1_LCTL_R            {() => (0x4000D090 as *mut u32);}
macro_rules! UART1_LSS_R             {() => (0x4000D094 as *mut u32);}
macro_rules! UART1_LTIM_R            {() => (0x4000D098 as *mut u32);}
macro_rules! UART1_9BITADDR_R        {() => (0x4000D0A4 as *mut u32);}
macro_rules! UART1_9BITAMASK_R       {() => (0x4000D0A8 as *mut u32);}
macro_rules! UART1_PP_R              {() => (0x4000DFC0 as *mut u32);}
macro_rules! UART1_CC_R              {() => (0x4000DFC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART2)
//
// *****************************************************************************
macro_rules! UART2_DR_R              {() => (0x4000E000 as *mut u32);}
macro_rules! UART2_RSR_R             {() => (0x4000E004 as *mut u32);}
macro_rules! UART2_ECR_R             {() => (0x4000E004 as *mut u32);}
macro_rules! UART2_FR_R              {() => (0x4000E018 as *mut u32);}
macro_rules! UART2_ILPR_R            {() => (0x4000E020 as *mut u32);}
macro_rules! UART2_IBRD_R            {() => (0x4000E024 as *mut u32);}
macro_rules! UART2_FBRD_R            {() => (0x4000E028 as *mut u32);}
macro_rules! UART2_LCRH_R            {() => (0x4000E02C as *mut u32);}
macro_rules! UART2_CTL_R             {() => (0x4000E030 as *mut u32);}
macro_rules! UART2_IFLS_R            {() => (0x4000E034 as *mut u32);}
macro_rules! UART2_IM_R              {() => (0x4000E038 as *mut u32);}
macro_rules! UART2_RIS_R             {() => (0x4000E03C as *mut u32);}
macro_rules! UART2_MIS_R             {() => (0x4000E040 as *mut u32);}
macro_rules! UART2_ICR_R             {() => (0x4000E044 as *mut u32);}
macro_rules! UART2_DMACTL_R          {() => (0x4000E048 as *mut u32);}
macro_rules! UART2_LCTL_R            {() => (0x4000E090 as *mut u32);}
macro_rules! UART2_LSS_R             {() => (0x4000E094 as *mut u32);}
macro_rules! UART2_LTIM_R            {() => (0x4000E098 as *mut u32);}
macro_rules! UART2_9BITADDR_R        {() => (0x4000E0A4 as *mut u32);}
macro_rules! UART2_9BITAMASK_R       {() => (0x4000E0A8 as *mut u32);}
macro_rules! UART2_PP_R              {() => (0x4000EFC0 as *mut u32);}
macro_rules! UART2_CC_R              {() => (0x4000EFC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART3)
//
// *****************************************************************************
macro_rules! UART3_DR_R              {() => (0x4000F000 as *mut u32);}
macro_rules! UART3_RSR_R             {() => (0x4000F004 as *mut u32);}
macro_rules! UART3_ECR_R             {() => (0x4000F004 as *mut u32);}
macro_rules! UART3_FR_R              {() => (0x4000F018 as *mut u32);}
macro_rules! UART3_ILPR_R            {() => (0x4000F020 as *mut u32);}
macro_rules! UART3_IBRD_R            {() => (0x4000F024 as *mut u32);}
macro_rules! UART3_FBRD_R            {() => (0x4000F028 as *mut u32);}
macro_rules! UART3_LCRH_R            {() => (0x4000F02C as *mut u32);}
macro_rules! UART3_CTL_R             {() => (0x4000F030 as *mut u32);}
macro_rules! UART3_IFLS_R            {() => (0x4000F034 as *mut u32);}
macro_rules! UART3_IM_R              {() => (0x4000F038 as *mut u32);}
macro_rules! UART3_RIS_R             {() => (0x4000F03C as *mut u32);}
macro_rules! UART3_MIS_R             {() => (0x4000F040 as *mut u32);}
macro_rules! UART3_ICR_R             {() => (0x4000F044 as *mut u32);}
macro_rules! UART3_DMACTL_R          {() => (0x4000F048 as *mut u32);}
macro_rules! UART3_LCTL_R            {() => (0x4000F090 as *mut u32);}
macro_rules! UART3_LSS_R             {() => (0x4000F094 as *mut u32);}
macro_rules! UART3_LTIM_R            {() => (0x4000F098 as *mut u32);}
macro_rules! UART3_9BITADDR_R        {() => (0x4000F0A4 as *mut u32);}
macro_rules! UART3_9BITAMASK_R       {() => (0x4000F0A8 as *mut u32);}
macro_rules! UART3_PP_R              {() => (0x4000FFC0 as *mut u32);}
macro_rules! UART3_CC_R              {() => (0x4000FFC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART4)
//
// *****************************************************************************
macro_rules! UART4_DR_R              {() => (0x40010000 as *mut u32);}
macro_rules! UART4_RSR_R             {() => (0x40010004 as *mut u32);}
macro_rules! UART4_ECR_R             {() => (0x40010004 as *mut u32);}
macro_rules! UART4_FR_R              {() => (0x40010018 as *mut u32);}
macro_rules! UART4_ILPR_R            {() => (0x40010020 as *mut u32);}
macro_rules! UART4_IBRD_R            {() => (0x40010024 as *mut u32);}
macro_rules! UART4_FBRD_R            {() => (0x40010028 as *mut u32);}
macro_rules! UART4_LCRH_R            {() => (0x4001002C as *mut u32);}
macro_rules! UART4_CTL_R             {() => (0x40010030 as *mut u32);}
macro_rules! UART4_IFLS_R            {() => (0x40010034 as *mut u32);}
macro_rules! UART4_IM_R              {() => (0x40010038 as *mut u32);}
macro_rules! UART4_RIS_R             {() => (0x4001003C as *mut u32);}
macro_rules! UART4_MIS_R             {() => (0x40010040 as *mut u32);}
macro_rules! UART4_ICR_R             {() => (0x40010044 as *mut u32);}
macro_rules! UART4_DMACTL_R          {() => (0x40010048 as *mut u32);}
macro_rules! UART4_LCTL_R            {() => (0x40010090 as *mut u32);}
macro_rules! UART4_LSS_R             {() => (0x40010094 as *mut u32);}
macro_rules! UART4_LTIM_R            {() => (0x40010098 as *mut u32);}
macro_rules! UART4_9BITADDR_R        {() => (0x400100A4 as *mut u32);}
macro_rules! UART4_9BITAMASK_R       {() => (0x400100A8 as *mut u32);}
macro_rules! UART4_PP_R              {() => (0x40010FC0 as *mut u32);}
macro_rules! UART4_CC_R              {() => (0x40010FC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART5)
//
// *****************************************************************************
macro_rules! UART5_DR_R              {() => (0x40011000 as *mut u32);}
macro_rules! UART5_RSR_R             {() => (0x40011004 as *mut u32);}
macro_rules! UART5_ECR_R             {() => (0x40011004 as *mut u32);}
macro_rules! UART5_FR_R              {() => (0x40011018 as *mut u32);}
macro_rules! UART5_ILPR_R            {() => (0x40011020 as *mut u32);}
macro_rules! UART5_IBRD_R            {() => (0x40011024 as *mut u32);}
macro_rules! UART5_FBRD_R            {() => (0x40011028 as *mut u32);}
macro_rules! UART5_LCRH_R            {() => (0x4001102C as *mut u32);}
macro_rules! UART5_CTL_R             {() => (0x40011030 as *mut u32);}
macro_rules! UART5_IFLS_R            {() => (0x40011034 as *mut u32);}
macro_rules! UART5_IM_R              {() => (0x40011038 as *mut u32);}
macro_rules! UART5_RIS_R             {() => (0x4001103C as *mut u32);}
macro_rules! UART5_MIS_R             {() => (0x40011040 as *mut u32);}
macro_rules! UART5_ICR_R             {() => (0x40011044 as *mut u32);}
macro_rules! UART5_DMACTL_R          {() => (0x40011048 as *mut u32);}
macro_rules! UART5_LCTL_R            {() => (0x40011090 as *mut u32);}
macro_rules! UART5_LSS_R             {() => (0x40011094 as *mut u32);}
macro_rules! UART5_LTIM_R            {() => (0x40011098 as *mut u32);}
macro_rules! UART5_9BITADDR_R        {() => (0x400110A4 as *mut u32);}
macro_rules! UART5_9BITAMASK_R       {() => (0x400110A8 as *mut u32);}
macro_rules! UART5_PP_R              {() => (0x40011FC0 as *mut u32);}
macro_rules! UART5_CC_R              {() => (0x40011FC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART6)
//
// *****************************************************************************
macro_rules! UART6_DR_R              {() => (0x40012000 as *mut u32);}
macro_rules! UART6_RSR_R             {() => (0x40012004 as *mut u32);}
macro_rules! UART6_ECR_R             {() => (0x40012004 as *mut u32);}
macro_rules! UART6_FR_R              {() => (0x40012018 as *mut u32);}
macro_rules! UART6_ILPR_R            {() => (0x40012020 as *mut u32);}
macro_rules! UART6_IBRD_R            {() => (0x40012024 as *mut u32);}
macro_rules! UART6_FBRD_R            {() => (0x40012028 as *mut u32);}
macro_rules! UART6_LCRH_R            {() => (0x4001202C as *mut u32);}
macro_rules! UART6_CTL_R             {() => (0x40012030 as *mut u32);}
macro_rules! UART6_IFLS_R            {() => (0x40012034 as *mut u32);}
macro_rules! UART6_IM_R              {() => (0x40012038 as *mut u32);}
macro_rules! UART6_RIS_R             {() => (0x4001203C as *mut u32);}
macro_rules! UART6_MIS_R             {() => (0x40012040 as *mut u32);}
macro_rules! UART6_ICR_R             {() => (0x40012044 as *mut u32);}
macro_rules! UART6_DMACTL_R          {() => (0x40012048 as *mut u32);}
macro_rules! UART6_LCTL_R            {() => (0x40012090 as *mut u32);}
macro_rules! UART6_LSS_R             {() => (0x40012094 as *mut u32);}
macro_rules! UART6_LTIM_R            {() => (0x40012098 as *mut u32);}
macro_rules! UART6_9BITADDR_R        {() => (0x400120A4 as *mut u32);}
macro_rules! UART6_9BITAMASK_R       {() => (0x400120A8 as *mut u32);}
macro_rules! UART6_PP_R              {() => (0x40012FC0 as *mut u32);}
macro_rules! UART6_CC_R              {() => (0x40012FC8 as *mut u32);}

// *****************************************************************************
//
// UART registers (UART7)
//
// *****************************************************************************
macro_rules! UART7_DR_R              {() => (0x40013000 as *mut u32);}
macro_rules! UART7_RSR_R             {() => (0x40013004 as *mut u32);}
macro_rules! UART7_ECR_R             {() => (0x40013004 as *mut u32);}
macro_rules! UART7_FR_R              {() => (0x40013018 as *mut u32);}
macro_rules! UART7_ILPR_R            {() => (0x40013020 as *mut u32);}
macro_rules! UART7_IBRD_R            {() => (0x40013024 as *mut u32);}
macro_rules! UART7_FBRD_R            {() => (0x40013028 as *mut u32);}
macro_rules! UART7_LCRH_R            {() => (0x4001302C as *mut u32);}
macro_rules! UART7_CTL_R             {() => (0x40013030 as *mut u32);}
macro_rules! UART7_IFLS_R            {() => (0x40013034 as *mut u32);}
macro_rules! UART7_IM_R              {() => (0x40013038 as *mut u32);}
macro_rules! UART7_RIS_R             {() => (0x4001303C as *mut u32);}
macro_rules! UART7_MIS_R             {() => (0x40013040 as *mut u32);}
macro_rules! UART7_ICR_R             {() => (0x40013044 as *mut u32);}
macro_rules! UART7_DMACTL_R          {() => (0x40013048 as *mut u32);}
macro_rules! UART7_LCTL_R            {() => (0x40013090 as *mut u32);}
macro_rules! UART7_LSS_R             {() => (0x40013094 as *mut u32);}
macro_rules! UART7_LTIM_R            {() => (0x40013098 as *mut u32);}
macro_rules! UART7_9BITADDR_R        {() => (0x400130A4 as *mut u32);}
macro_rules! UART7_9BITAMASK_R       {() => (0x400130A8 as *mut u32);}
macro_rules! UART7_PP_R              {() => (0x40013FC0 as *mut u32);}
macro_rules! UART7_CC_R              {() => (0x40013FC8 as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C0 MASTER)
//
// *****************************************************************************
macro_rules! I2C0_MASTER_MSA_R       {() => (0x40020000 as *mut u32);}
macro_rules! I2C0_MASTER_MCS_R       {() => (0x40020004 as *mut u32);}
macro_rules! I2C0_MASTER_MDR_R       {() => (0x40020008 as *mut u32);}
macro_rules! I2C0_MASTER_MTPR_R      {() => (0x4002000C as *mut u32);}
macro_rules! I2C0_MASTER_MIMR_R      {() => (0x40020010 as *mut u32);}
macro_rules! I2C0_MASTER_MRIS_R      {() => (0x40020014 as *mut u32);}
macro_rules! I2C0_MASTER_MMIS_R      {() => (0x40020018 as *mut u32);}
macro_rules! I2C0_MASTER_MICR_R      {() => (0x4002001C as *mut u32);}
macro_rules! I2C0_MASTER_MCR_R       {() => (0x40020020 as *mut u32);}
macro_rules! I2C0_MASTER_MCLKOCNT_R  {() => (0x40020024 as *mut u32);}
macro_rules! I2C0_MASTER_MBMON_R     {() => (0x4002002C as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C0 SLAVE)
//
// *****************************************************************************
macro_rules! I2C0_SLAVE_SOAR_R       {() => (0x40020800 as *mut u32);}
macro_rules! I2C0_SLAVE_SCSR_R       {() => (0x40020804 as *mut u32);}
macro_rules! I2C0_SLAVE_SDR_R        {() => (0x40020808 as *mut u32);}
macro_rules! I2C0_SLAVE_SIMR_R       {() => (0x4002080C as *mut u32);}
macro_rules! I2C0_SLAVE_SRIS_R       {() => (0x40020810 as *mut u32);}
macro_rules! I2C0_SLAVE_SMIS_R       {() => (0x40020814 as *mut u32);}
macro_rules! I2C0_SLAVE_SICR_R       {() => (0x40020818 as *mut u32);}
macro_rules! I2C0_SLAVE_SOAR2_R      {() => (0x4002081C as *mut u32);}
macro_rules! I2C0_SLAVE_SACKCTL_R    {() => (0x40020820 as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C1 MASTER)
//
// *****************************************************************************
macro_rules! I2C1_MASTER_MSA_R       {() => (0x40021000 as *mut u32);}
macro_rules! I2C1_MASTER_MCS_R       {() => (0x40021004 as *mut u32);}
macro_rules! I2C1_MASTER_MDR_R       {() => (0x40021008 as *mut u32);}
macro_rules! I2C1_MASTER_MTPR_R      {() => (0x4002100C as *mut u32);}
macro_rules! I2C1_MASTER_MIMR_R      {() => (0x40021010 as *mut u32);}
macro_rules! I2C1_MASTER_MRIS_R      {() => (0x40021014 as *mut u32);}
macro_rules! I2C1_MASTER_MMIS_R      {() => (0x40021018 as *mut u32);}
macro_rules! I2C1_MASTER_MICR_R      {() => (0x4002101C as *mut u32);}
macro_rules! I2C1_MASTER_MCR_R       {() => (0x40021020 as *mut u32);}
macro_rules! I2C1_MASTER_MCLKOCNT_R  {() => (0x40021024 as *mut u32);}
macro_rules! I2C1_MASTER_MBMON_R     {() => (0x4002102C as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C1 SLAVE)
//
// *****************************************************************************
macro_rules! I2C1_SLAVE_SOAR_R       {() => (0x40021800 as *mut u32);}
macro_rules! I2C1_SLAVE_SCSR_R       {() => (0x40021804 as *mut u32);}
macro_rules! I2C1_SLAVE_SDR_R        {() => (0x40021808 as *mut u32);}
macro_rules! I2C1_SLAVE_SIMR_R       {() => (0x4002180C as *mut u32);}
macro_rules! I2C1_SLAVE_SRIS_R       {() => (0x40021810 as *mut u32);}
macro_rules! I2C1_SLAVE_SMIS_R       {() => (0x40021814 as *mut u32);}
macro_rules! I2C1_SLAVE_SICR_R       {() => (0x40021818 as *mut u32);}
macro_rules! I2C1_SLAVE_SOAR2_R      {() => (0x4002181C as *mut u32);}
macro_rules! I2C1_SLAVE_SACKCTL_R    {() => (0x40021820 as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C2 MASTER)
//
// *****************************************************************************
macro_rules! I2C2_MASTER_MSA_R       {() => (0x40022000 as *mut u32);}
macro_rules! I2C2_MASTER_MCS_R       {() => (0x40022004 as *mut u32);}
macro_rules! I2C2_MASTER_MDR_R       {() => (0x40022008 as *mut u32);}
macro_rules! I2C2_MASTER_MTPR_R      {() => (0x4002200C as *mut u32);}
macro_rules! I2C2_MASTER_MIMR_R      {() => (0x40022010 as *mut u32);}
macro_rules! I2C2_MASTER_MRIS_R      {() => (0x40022014 as *mut u32);}
macro_rules! I2C2_MASTER_MMIS_R      {() => (0x40022018 as *mut u32);}
macro_rules! I2C2_MASTER_MICR_R      {() => (0x4002201C as *mut u32);}
macro_rules! I2C2_MASTER_MCR_R       {() => (0x40022020 as *mut u32);}
macro_rules! I2C2_MASTER_MCLKOCNT_R  {() => (0x40022024 as *mut u32);}
macro_rules! I2C2_MASTER_MBMON_R     {() => (0x4002202C as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C2 SLAVE)
//
// *****************************************************************************
macro_rules! I2C2_SLAVE_SOAR_R       {() => (0x40022800 as *mut u32);}
macro_rules! I2C2_SLAVE_SCSR_R       {() => (0x40022804 as *mut u32);}
macro_rules! I2C2_SLAVE_SDR_R        {() => (0x40022808 as *mut u32);}
macro_rules! I2C2_SLAVE_SIMR_R       {() => (0x4002280C as *mut u32);}
macro_rules! I2C2_SLAVE_SRIS_R       {() => (0x40022810 as *mut u32);}
macro_rules! I2C2_SLAVE_SMIS_R       {() => (0x40022814 as *mut u32);}
macro_rules! I2C2_SLAVE_SICR_R       {() => (0x40022818 as *mut u32);}
macro_rules! I2C2_SLAVE_SOAR2_R      {() => (0x4002281C as *mut u32);}
macro_rules! I2C2_SLAVE_SACKCTL_R    {() => (0x40022820 as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C3 MASTER)
//
// *****************************************************************************
macro_rules! I2C3_MASTER_MSA_R       {() => (0x40023000 as *mut u32);}
macro_rules! I2C3_MASTER_MCS_R       {() => (0x40023004 as *mut u32);}
macro_rules! I2C3_MASTER_MDR_R       {() => (0x40023008 as *mut u32);}
macro_rules! I2C3_MASTER_MTPR_R      {() => (0x4002300C as *mut u32);}
macro_rules! I2C3_MASTER_MIMR_R      {() => (0x40023010 as *mut u32);}
macro_rules! I2C3_MASTER_MRIS_R      {() => (0x40023014 as *mut u32);}
macro_rules! I2C3_MASTER_MMIS_R      {() => (0x40023018 as *mut u32);}
macro_rules! I2C3_MASTER_MICR_R      {() => (0x4002301C as *mut u32);}
macro_rules! I2C3_MASTER_MCR_R       {() => (0x40023020 as *mut u32);}
macro_rules! I2C3_MASTER_MCLKOCNT_R  {() => (0x40023024 as *mut u32);}
macro_rules! I2C3_MASTER_MBMON_R     {() => (0x4002302C as *mut u32);}

// *****************************************************************************
//
// I2C registers (I2C3 SLAVE)
//
// *****************************************************************************
macro_rules! I2C3_SLAVE_SOAR_R       {() => (0x40023800 as *mut u32);}
macro_rules! I2C3_SLAVE_SCSR_R       {() => (0x40023804 as *mut u32);}
macro_rules! I2C3_SLAVE_SDR_R        {() => (0x40023808 as *mut u32);}
macro_rules! I2C3_SLAVE_SIMR_R       {() => (0x4002380C as *mut u32);}
macro_rules! I2C3_SLAVE_SRIS_R       {() => (0x40023810 as *mut u32);}
macro_rules! I2C3_SLAVE_SMIS_R       {() => (0x40023814 as *mut u32);}
macro_rules! I2C3_SLAVE_SICR_R       {() => (0x40023818 as *mut u32);}
macro_rules! I2C3_SLAVE_SOAR2_R      {() => (0x4002381C as *mut u32);}
macro_rules! I2C3_SLAVE_SACKCTL_R    {() => (0x40023820 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTE)
//
// *****************************************************************************
macro_rules! GPIO_PORTE_DATA_BITS_R  {() => (0x40024000 as *mut u32);}
macro_rules! GPIO_PORTE_DATA_R       {() => (0x400243FC as *mut u32);}
macro_rules! GPIO_PORTE_DIR_R        {() => (0x40024400 as *mut u32);}
macro_rules! GPIO_PORTE_IS_R         {() => (0x40024404 as *mut u32);}
macro_rules! GPIO_PORTE_IBE_R        {() => (0x40024408 as *mut u32);}
macro_rules! GPIO_PORTE_IEV_R        {() => (0x4002440C as *mut u32);}
macro_rules! GPIO_PORTE_IM_R         {() => (0x40024410 as *mut u32);}
macro_rules! GPIO_PORTE_RIS_R        {() => (0x40024414 as *mut u32);}
macro_rules! GPIO_PORTE_MIS_R        {() => (0x40024418 as *mut u32);}
macro_rules! GPIO_PORTE_ICR_R        {() => (0x4002441C as *mut u32);}
macro_rules! GPIO_PORTE_AFSEL_R      {() => (0x40024420 as *mut u32);}
macro_rules! GPIO_PORTE_DR2R_R       {() => (0x40024500 as *mut u32);}
macro_rules! GPIO_PORTE_DR4R_R       {() => (0x40024504 as *mut u32);}
macro_rules! GPIO_PORTE_DR8R_R       {() => (0x40024508 as *mut u32);}
macro_rules! GPIO_PORTE_ODR_R        {() => (0x4002450C as *mut u32);}
macro_rules! GPIO_PORTE_PUR_R        {() => (0x40024510 as *mut u32);}
macro_rules! GPIO_PORTE_PDR_R        {() => (0x40024514 as *mut u32);}
macro_rules! GPIO_PORTE_SLR_R        {() => (0x40024518 as *mut u32);}
macro_rules! GPIO_PORTE_DEN_R        {() => (0x4002451C as *mut u32);}
macro_rules! GPIO_PORTE_LOCK_R       {() => (0x40024520 as *mut u32);}
macro_rules! GPIO_PORTE_CR_R         {() => (0x40024524 as *mut u32);}
macro_rules! GPIO_PORTE_AMSEL_R      {() => (0x40024528 as *mut u32);}
macro_rules! GPIO_PORTE_PCTL_R       {() => (0x4002452C as *mut u32);}
macro_rules! GPIO_PORTE_ADCCTL_R     {() => (0x40024530 as *mut u32);}
macro_rules! GPIO_PORTE_DMACTL_R     {() => (0x40024534 as *mut u32);}
macro_rules! GPIO_PORTE_SI_R         {() => (0x40024538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTF)
//
// *****************************************************************************
macro_rules! GPIO_PORTF_DATA_BITS_R  {() => (0x40025000 as *mut u32);}
macro_rules! GPIO_PORTF_DATA_R       {() => (0x400253FC as *mut u32);}
macro_rules! GPIO_PORTF_DIR_R        {() => (0x40025400 as *mut u32);}
macro_rules! GPIO_PORTF_IS_R         {() => (0x40025404 as *mut u32);}
macro_rules! GPIO_PORTF_IBE_R        {() => (0x40025408 as *mut u32);}
macro_rules! GPIO_PORTF_IEV_R        {() => (0x4002540C as *mut u32);}
macro_rules! GPIO_PORTF_IM_R         {() => (0x40025410 as *mut u32);}
macro_rules! GPIO_PORTF_RIS_R        {() => (0x40025414 as *mut u32);}
macro_rules! GPIO_PORTF_MIS_R        {() => (0x40025418 as *mut u32);}
macro_rules! GPIO_PORTF_ICR_R        {() => (0x4002541C as *mut u32);}
macro_rules! GPIO_PORTF_AFSEL_R      {() => (0x40025420 as *mut u32);}
macro_rules! GPIO_PORTF_DR2R_R       {() => (0x40025500 as *mut u32);}
macro_rules! GPIO_PORTF_DR4R_R       {() => (0x40025504 as *mut u32);}
macro_rules! GPIO_PORTF_DR8R_R       {() => (0x40025508 as *mut u32);}
macro_rules! GPIO_PORTF_ODR_R        {() => (0x4002550C as *mut u32);}
macro_rules! GPIO_PORTF_PUR_R        {() => (0x40025510 as *mut u32);}
macro_rules! GPIO_PORTF_PDR_R        {() => (0x40025514 as *mut u32);}
macro_rules! GPIO_PORTF_SLR_R        {() => (0x40025518 as *mut u32);}
macro_rules! GPIO_PORTF_DEN_R        {() => (0x4002551C as *mut u32);}
macro_rules! GPIO_PORTF_LOCK_R       {() => (0x40025520 as *mut u32);}
macro_rules! GPIO_PORTF_CR_R         {() => (0x40025524 as *mut u32);}
macro_rules! GPIO_PORTF_AMSEL_R      {() => (0x40025528 as *mut u32);}
macro_rules! GPIO_PORTF_PCTL_R       {() => (0x4002552C as *mut u32);}
macro_rules! GPIO_PORTF_ADCCTL_R     {() => (0x40025530 as *mut u32);}
macro_rules! GPIO_PORTF_DMACTL_R     {() => (0x40025534 as *mut u32);}
macro_rules! GPIO_PORTF_SI_R         {() => (0x40025538 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER0)
//
// *****************************************************************************
macro_rules! TIMER0_CFG_R            {() => (0x40030000 as *mut u32);}
macro_rules! TIMER0_TAMR_R           {() => (0x40030004 as *mut u32);}
macro_rules! TIMER0_TBMR_R           {() => (0x40030008 as *mut u32);}
macro_rules! TIMER0_CTL_R            {() => (0x4003000C as *mut u32);}
macro_rules! TIMER0_SYNC_R           {() => (0x40030010 as *mut u32);}
macro_rules! TIMER0_IMR_R            {() => (0x40030018 as *mut u32);}
macro_rules! TIMER0_RIS_R            {() => (0x4003001C as *mut u32);}
macro_rules! TIMER0_MIS_R            {() => (0x40030020 as *mut u32);}
macro_rules! TIMER0_ICR_R            {() => (0x40030024 as *mut u32);}
macro_rules! TIMER0_TAILR_R          {() => (0x40030028 as *mut u32);}
macro_rules! TIMER0_TBILR_R          {() => (0x4003002C as *mut u32);}
macro_rules! TIMER0_TAMATCHR_R       {() => (0x40030030 as *mut u32);}
macro_rules! TIMER0_TBMATCHR_R       {() => (0x40030034 as *mut u32);}
macro_rules! TIMER0_TAPR_R           {() => (0x40030038 as *mut u32);}
macro_rules! TIMER0_TBPR_R           {() => (0x4003003C as *mut u32);}
macro_rules! TIMER0_TAPMR_R          {() => (0x40030040 as *mut u32);}
macro_rules! TIMER0_TBPMR_R          {() => (0x40030044 as *mut u32);}
macro_rules! TIMER0_TAR_R            {() => (0x40030048 as *mut u32);}
macro_rules! TIMER0_TBR_R            {() => (0x4003004C as *mut u32);}
macro_rules! TIMER0_TAV_R            {() => (0x40030050 as *mut u32);}
macro_rules! TIMER0_TBV_R            {() => (0x40030054 as *mut u32);}
macro_rules! TIMER0_RTCPD_R          {() => (0x40030058 as *mut u32);}
macro_rules! TIMER0_TAPS_R           {() => (0x4003005C as *mut u32);}
macro_rules! TIMER0_TBPS_R           {() => (0x40030060 as *mut u32);}
macro_rules! TIMER0_TAPV_R           {() => (0x40030064 as *mut u32);}
macro_rules! TIMER0_TBPV_R           {() => (0x40030068 as *mut u32);}
macro_rules! TIMER0_PP_R             {() => (0x40030FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER1)
//
// *****************************************************************************
macro_rules! TIMER1_CFG_R            {() => (0x40031000 as *mut u32);}
macro_rules! TIMER1_TAMR_R           {() => (0x40031004 as *mut u32);}
macro_rules! TIMER1_TBMR_R           {() => (0x40031008 as *mut u32);}
macro_rules! TIMER1_CTL_R            {() => (0x4003100C as *mut u32);}
macro_rules! TIMER1_SYNC_R           {() => (0x40031010 as *mut u32);}
macro_rules! TIMER1_IMR_R            {() => (0x40031018 as *mut u32);}
macro_rules! TIMER1_RIS_R            {() => (0x4003101C as *mut u32);}
macro_rules! TIMER1_MIS_R            {() => (0x40031020 as *mut u32);}
macro_rules! TIMER1_ICR_R            {() => (0x40031024 as *mut u32);}
macro_rules! TIMER1_TAILR_R          {() => (0x40031028 as *mut u32);}
macro_rules! TIMER1_TBILR_R          {() => (0x4003102C as *mut u32);}
macro_rules! TIMER1_TAMATCHR_R       {() => (0x40031030 as *mut u32);}
macro_rules! TIMER1_TBMATCHR_R       {() => (0x40031034 as *mut u32);}
macro_rules! TIMER1_TAPR_R           {() => (0x40031038 as *mut u32);}
macro_rules! TIMER1_TBPR_R           {() => (0x4003103C as *mut u32);}
macro_rules! TIMER1_TAPMR_R          {() => (0x40031040 as *mut u32);}
macro_rules! TIMER1_TBPMR_R          {() => (0x40031044 as *mut u32);}
macro_rules! TIMER1_TAR_R            {() => (0x40031048 as *mut u32);}
macro_rules! TIMER1_TBR_R            {() => (0x4003104C as *mut u32);}
macro_rules! TIMER1_TAV_R            {() => (0x40031050 as *mut u32);}
macro_rules! TIMER1_TBV_R            {() => (0x40031054 as *mut u32);}
macro_rules! TIMER1_RTCPD_R          {() => (0x40031058 as *mut u32);}
macro_rules! TIMER1_TAPS_R           {() => (0x4003105C as *mut u32);}
macro_rules! TIMER1_TBPS_R           {() => (0x40031060 as *mut u32);}
macro_rules! TIMER1_TAPV_R           {() => (0x40031064 as *mut u32);}
macro_rules! TIMER1_TBPV_R           {() => (0x40031068 as *mut u32);}
macro_rules! TIMER1_PP_R             {() => (0x40031FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER2)
//
// *****************************************************************************
macro_rules! TIMER2_CFG_R            {() => (0x40032000 as *mut u32);}
macro_rules! TIMER2_TAMR_R           {() => (0x40032004 as *mut u32);}
macro_rules! TIMER2_TBMR_R           {() => (0x40032008 as *mut u32);}
macro_rules! TIMER2_CTL_R            {() => (0x4003200C as *mut u32);}
macro_rules! TIMER2_SYNC_R           {() => (0x40032010 as *mut u32);}
macro_rules! TIMER2_IMR_R            {() => (0x40032018 as *mut u32);}
macro_rules! TIMER2_RIS_R            {() => (0x4003201C as *mut u32);}
macro_rules! TIMER2_MIS_R            {() => (0x40032020 as *mut u32);}
macro_rules! TIMER2_ICR_R            {() => (0x40032024 as *mut u32);}
macro_rules! TIMER2_TAILR_R          {() => (0x40032028 as *mut u32);}
macro_rules! TIMER2_TBILR_R          {() => (0x4003202C as *mut u32);}
macro_rules! TIMER2_TAMATCHR_R       {() => (0x40032030 as *mut u32);}
macro_rules! TIMER2_TBMATCHR_R       {() => (0x40032034 as *mut u32);}
macro_rules! TIMER2_TAPR_R           {() => (0x40032038 as *mut u32);}
macro_rules! TIMER2_TBPR_R           {() => (0x4003203C as *mut u32);}
macro_rules! TIMER2_TAPMR_R          {() => (0x40032040 as *mut u32);}
macro_rules! TIMER2_TBPMR_R          {() => (0x40032044 as *mut u32);}
macro_rules! TIMER2_TAR_R            {() => (0x40032048 as *mut u32);}
macro_rules! TIMER2_TBR_R            {() => (0x4003204C as *mut u32);}
macro_rules! TIMER2_TAV_R            {() => (0x40032050 as *mut u32);}
macro_rules! TIMER2_TBV_R            {() => (0x40032054 as *mut u32);}
macro_rules! TIMER2_RTCPD_R          {() => (0x40032058 as *mut u32);}
macro_rules! TIMER2_TAPS_R           {() => (0x4003205C as *mut u32);}
macro_rules! TIMER2_TBPS_R           {() => (0x40032060 as *mut u32);}
macro_rules! TIMER2_TAPV_R           {() => (0x40032064 as *mut u32);}
macro_rules! TIMER2_TBPV_R           {() => (0x40032068 as *mut u32);}
macro_rules! TIMER2_PP_R             {() => (0x40032FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER3)
//
// *****************************************************************************
macro_rules! TIMER3_CFG_R            {() => (0x40033000 as *mut u32);}
macro_rules! TIMER3_TAMR_R           {() => (0x40033004 as *mut u32);}
macro_rules! TIMER3_TBMR_R           {() => (0x40033008 as *mut u32);}
macro_rules! TIMER3_CTL_R            {() => (0x4003300C as *mut u32);}
macro_rules! TIMER3_SYNC_R           {() => (0x40033010 as *mut u32);}
macro_rules! TIMER3_IMR_R            {() => (0x40033018 as *mut u32);}
macro_rules! TIMER3_RIS_R            {() => (0x4003301C as *mut u32);}
macro_rules! TIMER3_MIS_R            {() => (0x40033020 as *mut u32);}
macro_rules! TIMER3_ICR_R            {() => (0x40033024 as *mut u32);}
macro_rules! TIMER3_TAILR_R          {() => (0x40033028 as *mut u32);}
macro_rules! TIMER3_TBILR_R          {() => (0x4003302C as *mut u32);}
macro_rules! TIMER3_TAMATCHR_R       {() => (0x40033030 as *mut u32);}
macro_rules! TIMER3_TBMATCHR_R       {() => (0x40033034 as *mut u32);}
macro_rules! TIMER3_TAPR_R           {() => (0x40033038 as *mut u32);}
macro_rules! TIMER3_TBPR_R           {() => (0x4003303C as *mut u32);}
macro_rules! TIMER3_TAPMR_R          {() => (0x40033040 as *mut u32);}
macro_rules! TIMER3_TBPMR_R          {() => (0x40033044 as *mut u32);}
macro_rules! TIMER3_TAR_R            {() => (0x40033048 as *mut u32);}
macro_rules! TIMER3_TBR_R            {() => (0x4003304C as *mut u32);}
macro_rules! TIMER3_TAV_R            {() => (0x40033050 as *mut u32);}
macro_rules! TIMER3_TBV_R            {() => (0x40033054 as *mut u32);}
macro_rules! TIMER3_RTCPD_R          {() => (0x40033058 as *mut u32);}
macro_rules! TIMER3_TAPS_R           {() => (0x4003305C as *mut u32);}
macro_rules! TIMER3_TBPS_R           {() => (0x40033060 as *mut u32);}
macro_rules! TIMER3_TAPV_R           {() => (0x40033064 as *mut u32);}
macro_rules! TIMER3_TBPV_R           {() => (0x40033068 as *mut u32);}
macro_rules! TIMER3_PP_R             {() => (0x40033FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER4)
//
// *****************************************************************************
macro_rules! TIMER4_CFG_R            {() => (0x40034000 as *mut u32);}
macro_rules! TIMER4_TAMR_R           {() => (0x40034004 as *mut u32);}
macro_rules! TIMER4_TBMR_R           {() => (0x40034008 as *mut u32);}
macro_rules! TIMER4_CTL_R            {() => (0x4003400C as *mut u32);}
macro_rules! TIMER4_SYNC_R           {() => (0x40034010 as *mut u32);}
macro_rules! TIMER4_IMR_R            {() => (0x40034018 as *mut u32);}
macro_rules! TIMER4_RIS_R            {() => (0x4003401C as *mut u32);}
macro_rules! TIMER4_MIS_R            {() => (0x40034020 as *mut u32);}
macro_rules! TIMER4_ICR_R            {() => (0x40034024 as *mut u32);}
macro_rules! TIMER4_TAILR_R          {() => (0x40034028 as *mut u32);}
macro_rules! TIMER4_TBILR_R          {() => (0x4003402C as *mut u32);}
macro_rules! TIMER4_TAMATCHR_R       {() => (0x40034030 as *mut u32);}
macro_rules! TIMER4_TBMATCHR_R       {() => (0x40034034 as *mut u32);}
macro_rules! TIMER4_TAPR_R           {() => (0x40034038 as *mut u32);}
macro_rules! TIMER4_TBPR_R           {() => (0x4003403C as *mut u32);}
macro_rules! TIMER4_TAPMR_R          {() => (0x40034040 as *mut u32);}
macro_rules! TIMER4_TBPMR_R          {() => (0x40034044 as *mut u32);}
macro_rules! TIMER4_TAR_R            {() => (0x40034048 as *mut u32);}
macro_rules! TIMER4_TBR_R            {() => (0x4003404C as *mut u32);}
macro_rules! TIMER4_TAV_R            {() => (0x40034050 as *mut u32);}
macro_rules! TIMER4_TBV_R            {() => (0x40034054 as *mut u32);}
macro_rules! TIMER4_RTCPD_R          {() => (0x40034058 as *mut u32);}
macro_rules! TIMER4_TAPS_R           {() => (0x4003405C as *mut u32);}
macro_rules! TIMER4_TBPS_R           {() => (0x40034060 as *mut u32);}
macro_rules! TIMER4_TAPV_R           {() => (0x40034064 as *mut u32);}
macro_rules! TIMER4_TBPV_R           {() => (0x40034068 as *mut u32);}
macro_rules! TIMER4_PP_R             {() => (0x40034FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (TIMER5)
//
// *****************************************************************************
macro_rules! TIMER5_CFG_R            {() => (0x40035000 as *mut u32);}
macro_rules! TIMER5_TAMR_R           {() => (0x40035004 as *mut u32);}
macro_rules! TIMER5_TBMR_R           {() => (0x40035008 as *mut u32);}
macro_rules! TIMER5_CTL_R            {() => (0x4003500C as *mut u32);}
macro_rules! TIMER5_SYNC_R           {() => (0x40035010 as *mut u32);}
macro_rules! TIMER5_IMR_R            {() => (0x40035018 as *mut u32);}
macro_rules! TIMER5_RIS_R            {() => (0x4003501C as *mut u32);}
macro_rules! TIMER5_MIS_R            {() => (0x40035020 as *mut u32);}
macro_rules! TIMER5_ICR_R            {() => (0x40035024 as *mut u32);}
macro_rules! TIMER5_TAILR_R          {() => (0x40035028 as *mut u32);}
macro_rules! TIMER5_TBILR_R          {() => (0x4003502C as *mut u32);}
macro_rules! TIMER5_TAMATCHR_R       {() => (0x40035030 as *mut u32);}
macro_rules! TIMER5_TBMATCHR_R       {() => (0x40035034 as *mut u32);}
macro_rules! TIMER5_TAPR_R           {() => (0x40035038 as *mut u32);}
macro_rules! TIMER5_TBPR_R           {() => (0x4003503C as *mut u32);}
macro_rules! TIMER5_TAPMR_R          {() => (0x40035040 as *mut u32);}
macro_rules! TIMER5_TBPMR_R          {() => (0x40035044 as *mut u32);}
macro_rules! TIMER5_TAR_R            {() => (0x40035048 as *mut u32);}
macro_rules! TIMER5_TBR_R            {() => (0x4003504C as *mut u32);}
macro_rules! TIMER5_TAV_R            {() => (0x40035050 as *mut u32);}
macro_rules! TIMER5_TBV_R            {() => (0x40035054 as *mut u32);}
macro_rules! TIMER5_RTCPD_R          {() => (0x40035058 as *mut u32);}
macro_rules! TIMER5_TAPS_R           {() => (0x4003505C as *mut u32);}
macro_rules! TIMER5_TBPS_R           {() => (0x40035060 as *mut u32);}
macro_rules! TIMER5_TAPV_R           {() => (0x40035064 as *mut u32);}
macro_rules! TIMER5_TBPV_R           {() => (0x40035068 as *mut u32);}
macro_rules! TIMER5_PP_R             {() => (0x40035FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER0)
//
// *****************************************************************************
macro_rules! WTIMER0_CFG_R           {() => (0x40036000 as *mut u32);}
macro_rules! WTIMER0_TAMR_R          {() => (0x40036004 as *mut u32);}
macro_rules! WTIMER0_TBMR_R          {() => (0x40036008 as *mut u32);}
macro_rules! WTIMER0_CTL_R           {() => (0x4003600C as *mut u32);}
macro_rules! WTIMER0_SYNC_R          {() => (0x40036010 as *mut u32);}
macro_rules! WTIMER0_IMR_R           {() => (0x40036018 as *mut u32);}
macro_rules! WTIMER0_RIS_R           {() => (0x4003601C as *mut u32);}
macro_rules! WTIMER0_MIS_R           {() => (0x40036020 as *mut u32);}
macro_rules! WTIMER0_ICR_R           {() => (0x40036024 as *mut u32);}
macro_rules! WTIMER0_TAILR_R         {() => (0x40036028 as *mut u32);}
macro_rules! WTIMER0_TBILR_R         {() => (0x4003602C as *mut u32);}
macro_rules! WTIMER0_TAMATCHR_R      {() => (0x40036030 as *mut u32);}
macro_rules! WTIMER0_TBMATCHR_R      {() => (0x40036034 as *mut u32);}
macro_rules! WTIMER0_TAPR_R          {() => (0x40036038 as *mut u32);}
macro_rules! WTIMER0_TBPR_R          {() => (0x4003603C as *mut u32);}
macro_rules! WTIMER0_TAPMR_R         {() => (0x40036040 as *mut u32);}
macro_rules! WTIMER0_TBPMR_R         {() => (0x40036044 as *mut u32);}
macro_rules! WTIMER0_TAR_R           {() => (0x40036048 as *mut u32);}
macro_rules! WTIMER0_TBR_R           {() => (0x4003604C as *mut u32);}
macro_rules! WTIMER0_TAV_R           {() => (0x40036050 as *mut u32);}
macro_rules! WTIMER0_TBV_R           {() => (0x40036054 as *mut u32);}
macro_rules! WTIMER0_RTCPD_R         {() => (0x40036058 as *mut u32);}
macro_rules! WTIMER0_TAPS_R          {() => (0x4003605C as *mut u32);}
macro_rules! WTIMER0_TBPS_R          {() => (0x40036060 as *mut u32);}
macro_rules! WTIMER0_TAPV_R          {() => (0x40036064 as *mut u32);}
macro_rules! WTIMER0_TBPV_R          {() => (0x40036068 as *mut u32);}
macro_rules! WTIMER0_PP_R            {() => (0x40036FC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER1)
//
// *****************************************************************************
macro_rules! WTIMER1_CFG_R           {() => (0x40037000 as *mut u32);}
macro_rules! WTIMER1_TAMR_R          {() => (0x40037004 as *mut u32);}
macro_rules! WTIMER1_TBMR_R          {() => (0x40037008 as *mut u32);}
macro_rules! WTIMER1_CTL_R           {() => (0x4003700C as *mut u32);}
macro_rules! WTIMER1_SYNC_R          {() => (0x40037010 as *mut u32);}
macro_rules! WTIMER1_IMR_R           {() => (0x40037018 as *mut u32);}
macro_rules! WTIMER1_RIS_R           {() => (0x4003701C as *mut u32);}
macro_rules! WTIMER1_MIS_R           {() => (0x40037020 as *mut u32);}
macro_rules! WTIMER1_ICR_R           {() => (0x40037024 as *mut u32);}
macro_rules! WTIMER1_TAILR_R         {() => (0x40037028 as *mut u32);}
macro_rules! WTIMER1_TBILR_R         {() => (0x4003702C as *mut u32);}
macro_rules! WTIMER1_TAMATCHR_R      {() => (0x40037030 as *mut u32);}
macro_rules! WTIMER1_TBMATCHR_R      {() => (0x40037034 as *mut u32);}
macro_rules! WTIMER1_TAPR_R          {() => (0x40037038 as *mut u32);}
macro_rules! WTIMER1_TBPR_R          {() => (0x4003703C as *mut u32);}
macro_rules! WTIMER1_TAPMR_R         {() => (0x40037040 as *mut u32);}
macro_rules! WTIMER1_TBPMR_R         {() => (0x40037044 as *mut u32);}
macro_rules! WTIMER1_TAR_R           {() => (0x40037048 as *mut u32);}
macro_rules! WTIMER1_TBR_R           {() => (0x4003704C as *mut u32);}
macro_rules! WTIMER1_TAV_R           {() => (0x40037050 as *mut u32);}
macro_rules! WTIMER1_TBV_R           {() => (0x40037054 as *mut u32);}
macro_rules! WTIMER1_RTCPD_R         {() => (0x40037058 as *mut u32);}
macro_rules! WTIMER1_TAPS_R          {() => (0x4003705C as *mut u32);}
macro_rules! WTIMER1_TBPS_R          {() => (0x40037060 as *mut u32);}
macro_rules! WTIMER1_TAPV_R          {() => (0x40037064 as *mut u32);}
macro_rules! WTIMER1_TBPV_R          {() => (0x40037068 as *mut u32);}
macro_rules! WTIMER1_PP_R            {() => (0x40037FC0 as *mut u32);}

// *****************************************************************************
//
// ADC registers (ADC0)
//
// *****************************************************************************
macro_rules! ADC0_ACTSS_R            {() => (0x40038000 as *mut u32);}
macro_rules! ADC0_RIS_R              {() => (0x40038004 as *mut u32);}
macro_rules! ADC0_IM_R               {() => (0x40038008 as *mut u32);}
macro_rules! ADC0_ISC_R              {() => (0x4003800C as *mut u32);}
macro_rules! ADC0_OSTAT_R            {() => (0x40038010 as *mut u32);}
macro_rules! ADC0_EMUX_R             {() => (0x40038014 as *mut u32);}
macro_rules! ADC0_USTAT_R            {() => (0x40038018 as *mut u32);}
macro_rules! ADC0_SSPRI_R            {() => (0x40038020 as *mut u32);}
macro_rules! ADC0_SPC_R              {() => (0x40038024 as *mut u32);}
macro_rules! ADC0_PSSI_R             {() => (0x40038028 as *mut u32);}
macro_rules! ADC0_SAC_R              {() => (0x40038030 as *mut u32);}
macro_rules! ADC0_DCISC_R            {() => (0x40038034 as *mut u32);}
macro_rules! ADC0_SSMUX0_R           {() => (0x40038040 as *mut u32);}
macro_rules! ADC0_SSCTL0_R           {() => (0x40038044 as *mut u32);}
macro_rules! ADC0_SSFIFO0_R          {() => (0x40038048 as *mut u32);}
macro_rules! ADC0_SSFSTAT0_R         {() => (0x4003804C as *mut u32);}
macro_rules! ADC0_SSOP0_R            {() => (0x40038050 as *mut u32);}
macro_rules! ADC0_SSDC0_R            {() => (0x40038054 as *mut u32);}
macro_rules! ADC0_SSMUX1_R           {() => (0x40038060 as *mut u32);}
macro_rules! ADC0_SSCTL1_R           {() => (0x40038064 as *mut u32);}
macro_rules! ADC0_SSFIFO1_R          {() => (0x40038068 as *mut u32);}
macro_rules! ADC0_SSFSTAT1_R         {() => (0x4003806C as *mut u32);}
macro_rules! ADC0_SSOP1_R            {() => (0x40038070 as *mut u32);}
macro_rules! ADC0_SSDC1_R            {() => (0x40038074 as *mut u32);}
macro_rules! ADC0_SSMUX2_R           {() => (0x40038080 as *mut u32);}
macro_rules! ADC0_SSCTL2_R           {() => (0x40038084 as *mut u32);}
macro_rules! ADC0_SSFIFO2_R          {() => (0x40038088 as *mut u32);}
macro_rules! ADC0_SSFSTAT2_R         {() => (0x4003808C as *mut u32);}
macro_rules! ADC0_SSOP2_R            {() => (0x40038090 as *mut u32);}
macro_rules! ADC0_SSDC2_R            {() => (0x40038094 as *mut u32);}
macro_rules! ADC0_SSMUX3_R           {() => (0x400380A0 as *mut u32);}
macro_rules! ADC0_SSCTL3_R           {() => (0x400380A4 as *mut u32);}
macro_rules! ADC0_SSFIFO3_R          {() => (0x400380A8 as *mut u32);}
macro_rules! ADC0_SSFSTAT3_R         {() => (0x400380AC as *mut u32);}
macro_rules! ADC0_SSOP3_R            {() => (0x400380B0 as *mut u32);}
macro_rules! ADC0_SSDC3_R            {() => (0x400380B4 as *mut u32);}
macro_rules! ADC0_DCRIC_R            {() => (0x40038D00 as *mut u32);}
macro_rules! ADC0_DCCTL0_R           {() => (0x40038E00 as *mut u32);}
macro_rules! ADC0_DCCTL1_R           {() => (0x40038E04 as *mut u32);}
macro_rules! ADC0_DCCTL2_R           {() => (0x40038E08 as *mut u32);}
macro_rules! ADC0_DCCTL3_R           {() => (0x40038E0C as *mut u32);}
macro_rules! ADC0_DCCTL4_R           {() => (0x40038E10 as *mut u32);}
macro_rules! ADC0_DCCTL5_R           {() => (0x40038E14 as *mut u32);}
macro_rules! ADC0_DCCTL6_R           {() => (0x40038E18 as *mut u32);}
macro_rules! ADC0_DCCTL7_R           {() => (0x40038E1C as *mut u32);}
macro_rules! ADC0_DCCMP0_R           {() => (0x40038E40 as *mut u32);}
macro_rules! ADC0_DCCMP1_R           {() => (0x40038E44 as *mut u32);}
macro_rules! ADC0_DCCMP2_R           {() => (0x40038E48 as *mut u32);}
macro_rules! ADC0_DCCMP3_R           {() => (0x40038E4C as *mut u32);}
macro_rules! ADC0_DCCMP4_R           {() => (0x40038E50 as *mut u32);}
macro_rules! ADC0_DCCMP5_R           {() => (0x40038E54 as *mut u32);}
macro_rules! ADC0_DCCMP6_R           {() => (0x40038E58 as *mut u32);}
macro_rules! ADC0_DCCMP7_R           {() => (0x40038E5C as *mut u32);}
macro_rules! ADC0_PP_R               {() => (0x40038FC0 as *mut u32);}
macro_rules! ADC0_PC_R               {() => (0x40038FC4 as *mut u32);}
macro_rules! ADC0_CC_R               {() => (0x40038FC8 as *mut u32);}

// *****************************************************************************
//
// ADC registers (ADC1)
//
// *****************************************************************************
macro_rules! ADC1_ACTSS_R            {() => (0x40039000 as *mut u32);}
macro_rules! ADC1_RIS_R              {() => (0x40039004 as *mut u32);}
macro_rules! ADC1_IM_R               {() => (0x40039008 as *mut u32);}
macro_rules! ADC1_ISC_R              {() => (0x4003900C as *mut u32);}
macro_rules! ADC1_OSTAT_R            {() => (0x40039010 as *mut u32);}
macro_rules! ADC1_EMUX_R             {() => (0x40039014 as *mut u32);}
macro_rules! ADC1_USTAT_R            {() => (0x40039018 as *mut u32);}
macro_rules! ADC1_SSPRI_R            {() => (0x40039020 as *mut u32);}
macro_rules! ADC1_SPC_R              {() => (0x40039024 as *mut u32);}
macro_rules! ADC1_PSSI_R             {() => (0x40039028 as *mut u32);}
macro_rules! ADC1_SAC_R              {() => (0x40039030 as *mut u32);}
macro_rules! ADC1_DCISC_R            {() => (0x40039034 as *mut u32);}
macro_rules! ADC1_SSMUX0_R           {() => (0x40039040 as *mut u32);}
macro_rules! ADC1_SSCTL0_R           {() => (0x40039044 as *mut u32);}
macro_rules! ADC1_SSFIFO0_R          {() => (0x40039048 as *mut u32);}
macro_rules! ADC1_SSFSTAT0_R         {() => (0x4003904C as *mut u32);}
macro_rules! ADC1_SSOP0_R            {() => (0x40039050 as *mut u32);}
macro_rules! ADC1_SSDC0_R            {() => (0x40039054 as *mut u32);}
macro_rules! ADC1_SSMUX1_R           {() => (0x40039060 as *mut u32);}
macro_rules! ADC1_SSCTL1_R           {() => (0x40039064 as *mut u32);}
macro_rules! ADC1_SSFIFO1_R          {() => (0x40039068 as *mut u32);}
macro_rules! ADC1_SSFSTAT1_R         {() => (0x4003906C as *mut u32);}
macro_rules! ADC1_SSOP1_R            {() => (0x40039070 as *mut u32);}
macro_rules! ADC1_SSDC1_R            {() => (0x40039074 as *mut u32);}
macro_rules! ADC1_SSMUX2_R           {() => (0x40039080 as *mut u32);}
macro_rules! ADC1_SSCTL2_R           {() => (0x40039084 as *mut u32);}
macro_rules! ADC1_SSFIFO2_R          {() => (0x40039088 as *mut u32);}
macro_rules! ADC1_SSFSTAT2_R         {() => (0x4003908C as *mut u32);}
macro_rules! ADC1_SSOP2_R            {() => (0x40039090 as *mut u32);}
macro_rules! ADC1_SSDC2_R            {() => (0x40039094 as *mut u32);}
macro_rules! ADC1_SSMUX3_R           {() => (0x400390A0 as *mut u32);}
macro_rules! ADC1_SSCTL3_R           {() => (0x400390A4 as *mut u32);}
macro_rules! ADC1_SSFIFO3_R          {() => (0x400390A8 as *mut u32);}
macro_rules! ADC1_SSFSTAT3_R         {() => (0x400390AC as *mut u32);}
macro_rules! ADC1_SSOP3_R            {() => (0x400390B0 as *mut u32);}
macro_rules! ADC1_SSDC3_R            {() => (0x400390B4 as *mut u32);}
macro_rules! ADC1_DCRIC_R            {() => (0x40039D00 as *mut u32);}
macro_rules! ADC1_DCCTL0_R           {() => (0x40039E00 as *mut u32);}
macro_rules! ADC1_DCCTL1_R           {() => (0x40039E04 as *mut u32);}
macro_rules! ADC1_DCCTL2_R           {() => (0x40039E08 as *mut u32);}
macro_rules! ADC1_DCCTL3_R           {() => (0x40039E0C as *mut u32);}
macro_rules! ADC1_DCCTL4_R           {() => (0x40039E10 as *mut u32);}
macro_rules! ADC1_DCCTL5_R           {() => (0x40039E14 as *mut u32);}
macro_rules! ADC1_DCCTL6_R           {() => (0x40039E18 as *mut u32);}
macro_rules! ADC1_DCCTL7_R           {() => (0x40039E1C as *mut u32);}
macro_rules! ADC1_DCCMP0_R           {() => (0x40039E40 as *mut u32);}
macro_rules! ADC1_DCCMP1_R           {() => (0x40039E44 as *mut u32);}
macro_rules! ADC1_DCCMP2_R           {() => (0x40039E48 as *mut u32);}
macro_rules! ADC1_DCCMP3_R           {() => (0x40039E4C as *mut u32);}
macro_rules! ADC1_DCCMP4_R           {() => (0x40039E50 as *mut u32);}
macro_rules! ADC1_DCCMP5_R           {() => (0x40039E54 as *mut u32);}
macro_rules! ADC1_DCCMP6_R           {() => (0x40039E58 as *mut u32);}
macro_rules! ADC1_DCCMP7_R           {() => (0x40039E5C as *mut u32);}
macro_rules! ADC1_PP_R               {() => (0x40039FC0 as *mut u32);}
macro_rules! ADC1_PC_R               {() => (0x40039FC4 as *mut u32);}
macro_rules! ADC1_CC_R               {() => (0x40039FC8 as *mut u32);}

// *****************************************************************************
//
// Comparator registers (COMP)
//
// *****************************************************************************
macro_rules! COMP_ACMIS_R            {() => (0x4003C000 as *mut u32);}
macro_rules! COMP_ACRIS_R            {() => (0x4003C004 as *mut u32);}
macro_rules! COMP_ACINTEN_R          {() => (0x4003C008 as *mut u32);}
macro_rules! COMP_ACREFCTL_R         {() => (0x4003C010 as *mut u32);}
macro_rules! COMP_ACSTAT0_R          {() => (0x4003C020 as *mut u32);}
macro_rules! COMP_ACCTL0_R           {() => (0x4003C024 as *mut u32);}
macro_rules! COMP_ACSTAT1_R          {() => (0x4003C040 as *mut u32);}
macro_rules! COMP_ACCTL1_R           {() => (0x4003C044 as *mut u32);}
macro_rules! COMP_PP_R               {() => (0x4003CFC0 as *mut u32);}

// *****************************************************************************
//
// CAN registers (CAN0)
//
// *****************************************************************************
macro_rules! CAN0_CTL_R              {() => (0x40040000 as *mut u32);}
macro_rules! CAN0_STS_R              {() => (0x40040004 as *mut u32);}
macro_rules! CAN0_ERR_R              {() => (0x40040008 as *mut u32);}
macro_rules! CAN0_BIT_R              {() => (0x4004000C as *mut u32);}
macro_rules! CAN0_INT_R              {() => (0x40040010 as *mut u32);}
macro_rules! CAN0_TST_R              {() => (0x40040014 as *mut u32);}
macro_rules! CAN0_BRPE_R             {() => (0x40040018 as *mut u32);}
macro_rules! CAN0_IF1CRQ_R           {() => (0x40040020 as *mut u32);}
macro_rules! CAN0_IF1CMSK_R          {() => (0x40040024 as *mut u32);}
macro_rules! CAN0_IF1MSK1_R          {() => (0x40040028 as *mut u32);}
macro_rules! CAN0_IF1MSK2_R          {() => (0x4004002C as *mut u32);}
macro_rules! CAN0_IF1ARB1_R          {() => (0x40040030 as *mut u32);}
macro_rules! CAN0_IF1ARB2_R          {() => (0x40040034 as *mut u32);}
macro_rules! CAN0_IF1MCTL_R          {() => (0x40040038 as *mut u32);}
macro_rules! CAN0_IF1DA1_R           {() => (0x4004003C as *mut u32);}
macro_rules! CAN0_IF1DA2_R           {() => (0x40040040 as *mut u32);}
macro_rules! CAN0_IF1DB1_R           {() => (0x40040044 as *mut u32);}
macro_rules! CAN0_IF1DB2_R           {() => (0x40040048 as *mut u32);}
macro_rules! CAN0_IF2CRQ_R           {() => (0x40040080 as *mut u32);}
macro_rules! CAN0_IF2CMSK_R          {() => (0x40040084 as *mut u32);}
macro_rules! CAN0_IF2MSK1_R          {() => (0x40040088 as *mut u32);}
macro_rules! CAN0_IF2MSK2_R          {() => (0x4004008C as *mut u32);}
macro_rules! CAN0_IF2ARB1_R          {() => (0x40040090 as *mut u32);}
macro_rules! CAN0_IF2ARB2_R          {() => (0x40040094 as *mut u32);}
macro_rules! CAN0_IF2MCTL_R          {() => (0x40040098 as *mut u32);}
macro_rules! CAN0_IF2DA1_R           {() => (0x4004009C as *mut u32);}
macro_rules! CAN0_IF2DA2_R           {() => (0x400400A0 as *mut u32);}
macro_rules! CAN0_IF2DB1_R           {() => (0x400400A4 as *mut u32);}
macro_rules! CAN0_IF2DB2_R           {() => (0x400400A8 as *mut u32);}
macro_rules! CAN0_TXRQ1_R            {() => (0x40040100 as *mut u32);}
macro_rules! CAN0_TXRQ2_R            {() => (0x40040104 as *mut u32);}
macro_rules! CAN0_NWDA1_R            {() => (0x40040120 as *mut u32);}
macro_rules! CAN0_NWDA2_R            {() => (0x40040124 as *mut u32);}
macro_rules! CAN0_MSG1INT_R          {() => (0x40040140 as *mut u32);}
macro_rules! CAN0_MSG2INT_R          {() => (0x40040144 as *mut u32);}
macro_rules! CAN0_MSG1VAL_R          {() => (0x40040160 as *mut u32);}
macro_rules! CAN0_MSG2VAL_R          {() => (0x40040164 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER2)
//
// *****************************************************************************
macro_rules! WTIMER2_CFG_R           {() => (0x4004C000 as *mut u32);}
macro_rules! WTIMER2_TAMR_R          {() => (0x4004C004 as *mut u32);}
macro_rules! WTIMER2_TBMR_R          {() => (0x4004C008 as *mut u32);}
macro_rules! WTIMER2_CTL_R           {() => (0x4004C00C as *mut u32);}
macro_rules! WTIMER2_SYNC_R          {() => (0x4004C010 as *mut u32);}
macro_rules! WTIMER2_IMR_R           {() => (0x4004C018 as *mut u32);}
macro_rules! WTIMER2_RIS_R           {() => (0x4004C01C as *mut u32);}
macro_rules! WTIMER2_MIS_R           {() => (0x4004C020 as *mut u32);}
macro_rules! WTIMER2_ICR_R           {() => (0x4004C024 as *mut u32);}
macro_rules! WTIMER2_TAILR_R         {() => (0x4004C028 as *mut u32);}
macro_rules! WTIMER2_TBILR_R         {() => (0x4004C02C as *mut u32);}
macro_rules! WTIMER2_TAMATCHR_R      {() => (0x4004C030 as *mut u32);}
macro_rules! WTIMER2_TBMATCHR_R      {() => (0x4004C034 as *mut u32);}
macro_rules! WTIMER2_TAPR_R          {() => (0x4004C038 as *mut u32);}
macro_rules! WTIMER2_TBPR_R          {() => (0x4004C03C as *mut u32);}
macro_rules! WTIMER2_TAPMR_R         {() => (0x4004C040 as *mut u32);}
macro_rules! WTIMER2_TBPMR_R         {() => (0x4004C044 as *mut u32);}
macro_rules! WTIMER2_TAR_R           {() => (0x4004C048 as *mut u32);}
macro_rules! WTIMER2_TBR_R           {() => (0x4004C04C as *mut u32);}
macro_rules! WTIMER2_TAV_R           {() => (0x4004C050 as *mut u32);}
macro_rules! WTIMER2_TBV_R           {() => (0x4004C054 as *mut u32);}
macro_rules! WTIMER2_RTCPD_R         {() => (0x4004C058 as *mut u32);}
macro_rules! WTIMER2_TAPS_R          {() => (0x4004C05C as *mut u32);}
macro_rules! WTIMER2_TBPS_R          {() => (0x4004C060 as *mut u32);}
macro_rules! WTIMER2_TAPV_R          {() => (0x4004C064 as *mut u32);}
macro_rules! WTIMER2_TBPV_R          {() => (0x4004C068 as *mut u32);}
macro_rules! WTIMER2_PP_R            {() => (0x4004CFC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER3)
//
// *****************************************************************************
macro_rules! WTIMER3_CFG_R           {() => (0x4004D000 as *mut u32);}
macro_rules! WTIMER3_TAMR_R          {() => (0x4004D004 as *mut u32);}
macro_rules! WTIMER3_TBMR_R          {() => (0x4004D008 as *mut u32);}
macro_rules! WTIMER3_CTL_R           {() => (0x4004D00C as *mut u32);}
macro_rules! WTIMER3_SYNC_R          {() => (0x4004D010 as *mut u32);}
macro_rules! WTIMER3_IMR_R           {() => (0x4004D018 as *mut u32);}
macro_rules! WTIMER3_RIS_R           {() => (0x4004D01C as *mut u32);}
macro_rules! WTIMER3_MIS_R           {() => (0x4004D020 as *mut u32);}
macro_rules! WTIMER3_ICR_R           {() => (0x4004D024 as *mut u32);}
macro_rules! WTIMER3_TAILR_R         {() => (0x4004D028 as *mut u32);}
macro_rules! WTIMER3_TBILR_R         {() => (0x4004D02C as *mut u32);}
macro_rules! WTIMER3_TAMATCHR_R      {() => (0x4004D030 as *mut u32);}
macro_rules! WTIMER3_TBMATCHR_R      {() => (0x4004D034 as *mut u32);}
macro_rules! WTIMER3_TAPR_R          {() => (0x4004D038 as *mut u32);}
macro_rules! WTIMER3_TBPR_R          {() => (0x4004D03C as *mut u32);}
macro_rules! WTIMER3_TAPMR_R         {() => (0x4004D040 as *mut u32);}
macro_rules! WTIMER3_TBPMR_R         {() => (0x4004D044 as *mut u32);}
macro_rules! WTIMER3_TAR_R           {() => (0x4004D048 as *mut u32);}
macro_rules! WTIMER3_TBR_R           {() => (0x4004D04C as *mut u32);}
macro_rules! WTIMER3_TAV_R           {() => (0x4004D050 as *mut u32);}
macro_rules! WTIMER3_TBV_R           {() => (0x4004D054 as *mut u32);}
macro_rules! WTIMER3_RTCPD_R         {() => (0x4004D058 as *mut u32);}
macro_rules! WTIMER3_TAPS_R          {() => (0x4004D05C as *mut u32);}
macro_rules! WTIMER3_TBPS_R          {() => (0x4004D060 as *mut u32);}
macro_rules! WTIMER3_TAPV_R          {() => (0x4004D064 as *mut u32);}
macro_rules! WTIMER3_TBPV_R          {() => (0x4004D068 as *mut u32);}
macro_rules! WTIMER3_PP_R            {() => (0x4004DFC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER4)
//
// *****************************************************************************
macro_rules! WTIMER4_CFG_R           {() => (0x4004E000 as *mut u32);}
macro_rules! WTIMER4_TAMR_R          {() => (0x4004E004 as *mut u32);}
macro_rules! WTIMER4_TBMR_R          {() => (0x4004E008 as *mut u32);}
macro_rules! WTIMER4_CTL_R           {() => (0x4004E00C as *mut u32);}
macro_rules! WTIMER4_SYNC_R          {() => (0x4004E010 as *mut u32);}
macro_rules! WTIMER4_IMR_R           {() => (0x4004E018 as *mut u32);}
macro_rules! WTIMER4_RIS_R           {() => (0x4004E01C as *mut u32);}
macro_rules! WTIMER4_MIS_R           {() => (0x4004E020 as *mut u32);}
macro_rules! WTIMER4_ICR_R           {() => (0x4004E024 as *mut u32);}
macro_rules! WTIMER4_TAILR_R         {() => (0x4004E028 as *mut u32);}
macro_rules! WTIMER4_TBILR_R         {() => (0x4004E02C as *mut u32);}
macro_rules! WTIMER4_TAMATCHR_R      {() => (0x4004E030 as *mut u32);}
macro_rules! WTIMER4_TBMATCHR_R      {() => (0x4004E034 as *mut u32);}
macro_rules! WTIMER4_TAPR_R          {() => (0x4004E038 as *mut u32);}
macro_rules! WTIMER4_TBPR_R          {() => (0x4004E03C as *mut u32);}
macro_rules! WTIMER4_TAPMR_R         {() => (0x4004E040 as *mut u32);}
macro_rules! WTIMER4_TBPMR_R         {() => (0x4004E044 as *mut u32);}
macro_rules! WTIMER4_TAR_R           {() => (0x4004E048 as *mut u32);}
macro_rules! WTIMER4_TBR_R           {() => (0x4004E04C as *mut u32);}
macro_rules! WTIMER4_TAV_R           {() => (0x4004E050 as *mut u32);}
macro_rules! WTIMER4_TBV_R           {() => (0x4004E054 as *mut u32);}
macro_rules! WTIMER4_RTCPD_R         {() => (0x4004E058 as *mut u32);}
macro_rules! WTIMER4_TAPS_R          {() => (0x4004E05C as *mut u32);}
macro_rules! WTIMER4_TBPS_R          {() => (0x4004E060 as *mut u32);}
macro_rules! WTIMER4_TAPV_R          {() => (0x4004E064 as *mut u32);}
macro_rules! WTIMER4_TBPV_R          {() => (0x4004E068 as *mut u32);}
macro_rules! WTIMER4_PP_R            {() => (0x4004EFC0 as *mut u32);}

// *****************************************************************************
//
// Timer registers (WTIMER5)
//
// *****************************************************************************
macro_rules! WTIMER5_CFG_R           {() => (0x4004F000 as *mut u32);}
macro_rules! WTIMER5_TAMR_R          {() => (0x4004F004 as *mut u32);}
macro_rules! WTIMER5_TBMR_R          {() => (0x4004F008 as *mut u32);}
macro_rules! WTIMER5_CTL_R           {() => (0x4004F00C as *mut u32);}
macro_rules! WTIMER5_SYNC_R          {() => (0x4004F010 as *mut u32);}
macro_rules! WTIMER5_IMR_R           {() => (0x4004F018 as *mut u32);}
macro_rules! WTIMER5_RIS_R           {() => (0x4004F01C as *mut u32);}
macro_rules! WTIMER5_MIS_R           {() => (0x4004F020 as *mut u32);}
macro_rules! WTIMER5_ICR_R           {() => (0x4004F024 as *mut u32);}
macro_rules! WTIMER5_TAILR_R         {() => (0x4004F028 as *mut u32);}
macro_rules! WTIMER5_TBILR_R         {() => (0x4004F02C as *mut u32);}
macro_rules! WTIMER5_TAMATCHR_R      {() => (0x4004F030 as *mut u32);}
macro_rules! WTIMER5_TBMATCHR_R      {() => (0x4004F034 as *mut u32);}
macro_rules! WTIMER5_TAPR_R          {() => (0x4004F038 as *mut u32);}
macro_rules! WTIMER5_TBPR_R          {() => (0x4004F03C as *mut u32);}
macro_rules! WTIMER5_TAPMR_R         {() => (0x4004F040 as *mut u32);}
macro_rules! WTIMER5_TBPMR_R         {() => (0x4004F044 as *mut u32);}
macro_rules! WTIMER5_TAR_R           {() => (0x4004F048 as *mut u32);}
macro_rules! WTIMER5_TBR_R           {() => (0x4004F04C as *mut u32);}
macro_rules! WTIMER5_TAV_R           {() => (0x4004F050 as *mut u32);}
macro_rules! WTIMER5_TBV_R           {() => (0x4004F054 as *mut u32);}
macro_rules! WTIMER5_RTCPD_R         {() => (0x4004F058 as *mut u32);}
macro_rules! WTIMER5_TAPS_R          {() => (0x4004F05C as *mut u32);}
macro_rules! WTIMER5_TBPS_R          {() => (0x4004F060 as *mut u32);}
macro_rules! WTIMER5_TAPV_R          {() => (0x4004F064 as *mut u32);}
macro_rules! WTIMER5_TBPV_R          {() => (0x4004F068 as *mut u32);}
macro_rules! WTIMER5_PP_R            {() => (0x4004FFC0 as *mut u32);}

// *****************************************************************************
//
// Univeral Serial Bus registers (USB0)
//
// *****************************************************************************
macro_rules! USB0_FADDR_R            {() => (0x40050000 as *mut u8);}
macro_rules! USB0_POWER_R            {() => (0x40050001 as *mut u8);}
macro_rules! USB0_TXIS_R             {() => (0x40050002 as *mut u16);}
macro_rules! USB0_RXIS_R             {() => (0x40050004 as *mut u16);}
macro_rules! USB0_TXIE_R             {() => (0x40050006 as *mut u16);}
macro_rules! USB0_RXIE_R             {() => (0x40050008 as *mut u16);}
macro_rules! USB0_IS_R               {() => (0x4005000A as *mut u8);}
macro_rules! USB0_IE_R               {() => (0x4005000B as *mut u8);}
macro_rules! USB0_FRAME_R            {() => (0x4005000C as *mut u16);}
macro_rules! USB0_EPIDX_R            {() => (0x4005000E as *mut u8);}
macro_rules! USB0_TEST_R             {() => (0x4005000F as *mut u8);}
macro_rules! USB0_FIFO0_R            {() => (0x40050020 as *mut u32);}
macro_rules! USB0_FIFO1_R            {() => (0x40050024 as *mut u32);}
macro_rules! USB0_FIFO2_R            {() => (0x40050028 as *mut u32);}
macro_rules! USB0_FIFO3_R            {() => (0x4005002C as *mut u32);}
macro_rules! USB0_FIFO4_R            {() => (0x40050030 as *mut u32);}
macro_rules! USB0_FIFO5_R            {() => (0x40050034 as *mut u32);}
macro_rules! USB0_FIFO6_R            {() => (0x40050038 as *mut u32);}
macro_rules! USB0_FIFO7_R            {() => (0x4005003C as *mut u32);}
macro_rules! USB0_TXFIFOSZ_R         {() => (0x40050062 as *mut u8);}
macro_rules! USB0_RXFIFOSZ_R         {() => (0x40050063 as *mut u8);}
macro_rules! USB0_TXFIFOADD_R        {() => (0x40050064 as *mut u16);}
macro_rules! USB0_RXFIFOADD_R        {() => (0x40050066 as *mut u16);}
macro_rules! USB0_CONTIM_R           {() => (0x4005007A as *mut u8);}
macro_rules! USB0_FSEOF_R            {() => (0x4005007D as *mut u8);}
macro_rules! USB0_LSEOF_R            {() => (0x4005007E as *mut u8);}
macro_rules! USB0_CSRL0_R            {() => (0x40050102 as *mut u8);}
macro_rules! USB0_CSRH0_R            {() => (0x40050103 as *mut u8);}
macro_rules! USB0_COUNT0_R           {() => (0x40050108 as *mut u8);}
macro_rules! USB0_TXMAXP1_R          {() => (0x40050110 as *mut u16);}
macro_rules! USB0_TXCSRL1_R          {() => (0x40050112 as *mut u8);}
macro_rules! USB0_TXCSRH1_R          {() => (0x40050113 as *mut u8);}
macro_rules! USB0_RXMAXP1_R          {() => (0x40050114 as *mut u16);}
macro_rules! USB0_RXCSRL1_R          {() => (0x40050116 as *mut u8);}
macro_rules! USB0_RXCSRH1_R          {() => (0x40050117 as *mut u8);}
macro_rules! USB0_RXCOUNT1_R         {() => (0x40050118 as *mut u16);}
macro_rules! USB0_TXMAXP2_R          {() => (0x40050120 as *mut u16);}
macro_rules! USB0_TXCSRL2_R          {() => (0x40050122 as *mut u8);}
macro_rules! USB0_TXCSRH2_R          {() => (0x40050123 as *mut u8);}
macro_rules! USB0_RXMAXP2_R          {() => (0x40050124 as *mut u16);}
macro_rules! USB0_RXCSRL2_R          {() => (0x40050126 as *mut u8);}
macro_rules! USB0_RXCSRH2_R          {() => (0x40050127 as *mut u8);}
macro_rules! USB0_RXCOUNT2_R         {() => (0x40050128 as *mut u16);}
macro_rules! USB0_TXMAXP3_R          {() => (0x40050130 as *mut u16);}
macro_rules! USB0_TXCSRL3_R          {() => (0x40050132 as *mut u8);}
macro_rules! USB0_TXCSRH3_R          {() => (0x40050133 as *mut u8);}
macro_rules! USB0_RXMAXP3_R          {() => (0x40050134 as *mut u16);}
macro_rules! USB0_RXCSRL3_R          {() => (0x40050136 as *mut u8);}
macro_rules! USB0_RXCSRH3_R          {() => (0x40050137 as *mut u8);}
macro_rules! USB0_RXCOUNT3_R         {() => (0x40050138 as *mut u16);}
macro_rules! USB0_TXMAXP4_R          {() => (0x40050140 as *mut u16);}
macro_rules! USB0_TXCSRL4_R          {() => (0x40050142 as *mut u8);}
macro_rules! USB0_TXCSRH4_R          {() => (0x40050143 as *mut u8);}
macro_rules! USB0_RXMAXP4_R          {() => (0x40050144 as *mut u16);}
macro_rules! USB0_RXCSRL4_R          {() => (0x40050146 as *mut u8);}
macro_rules! USB0_RXCSRH4_R          {() => (0x40050147 as *mut u8);}
macro_rules! USB0_RXCOUNT4_R         {() => (0x40050148 as *mut u16);}
macro_rules! USB0_TXMAXP5_R          {() => (0x40050150 as *mut u16);}
macro_rules! USB0_TXCSRL5_R          {() => (0x40050152 as *mut u8);}
macro_rules! USB0_TXCSRH5_R          {() => (0x40050153 as *mut u8);}
macro_rules! USB0_RXMAXP5_R          {() => (0x40050154 as *mut u16);}
macro_rules! USB0_RXCSRL5_R          {() => (0x40050156 as *mut u8);}
macro_rules! USB0_RXCSRH5_R          {() => (0x40050157 as *mut u8);}
macro_rules! USB0_RXCOUNT5_R         {() => (0x40050158 as *mut u16);}
macro_rules! USB0_TXMAXP6_R          {() => (0x40050160 as *mut u16);}
macro_rules! USB0_TXCSRL6_R          {() => (0x40050162 as *mut u8);}
macro_rules! USB0_TXCSRH6_R          {() => (0x40050163 as *mut u8);}
macro_rules! USB0_RXMAXP6_R          {() => (0x40050164 as *mut u16);}
macro_rules! USB0_RXCSRL6_R          {() => (0x40050166 as *mut u8);}
macro_rules! USB0_RXCSRH6_R          {() => (0x40050167 as *mut u8);}
macro_rules! USB0_RXCOUNT6_R         {() => (0x40050168 as *mut u16);}
macro_rules! USB0_TXMAXP7_R          {() => (0x40050170 as *mut u16);}
macro_rules! USB0_TXCSRL7_R          {() => (0x40050172 as *mut u8);}
macro_rules! USB0_TXCSRH7_R          {() => (0x40050173 as *mut u8);}
macro_rules! USB0_RXMAXP7_R          {() => (0x40050174 as *mut u16);}
macro_rules! USB0_RXCSRL7_R          {() => (0x40050176 as *mut u8);}
macro_rules! USB0_RXCSRH7_R          {() => (0x40050177 as *mut u8);}
macro_rules! USB0_RXCOUNT7_R         {() => (0x40050178 as *mut u16);}
macro_rules! USB0_RXDPKTBUFDIS_R     {() => (0x40050340 as *mut u16);}
macro_rules! USB0_TXDPKTBUFDIS_R     {() => (0x40050342 as *mut u16);}
macro_rules! USB0_DRRIS_R            {() => (0x40050410 as *mut u32);}
macro_rules! USB0_DRIM_R             {() => (0x40050414 as *mut u32);}
macro_rules! USB0_DRISC_R            {() => (0x40050418 as *mut u32);}
macro_rules! USB0_DMASEL_R           {() => (0x40050450 as *mut u32);}
macro_rules! USB0_PP_R               {() => (0x40050FC0 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTA AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTA_AHB_DATA_BITS_R {() => (0x40058000 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DATA_R   {() => (0x400583FC as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DIR_R    {() => (0x40058400 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_IS_R     {() => (0x40058404 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_IBE_R    {() => (0x40058408 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_IEV_R    {() => (0x4005840C as *mut u32);}
macro_rules! GPIO_PORTA_AHB_IM_R     {() => (0x40058410 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_RIS_R    {() => (0x40058414 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_MIS_R    {() => (0x40058418 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_ICR_R    {() => (0x4005841C as *mut u32);}
macro_rules! GPIO_PORTA_AHB_AFSEL_R  {() => (0x40058420 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DR2R_R   {() => (0x40058500 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DR4R_R   {() => (0x40058504 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DR8R_R   {() => (0x40058508 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_ODR_R    {() => (0x4005850C as *mut u32);}
macro_rules! GPIO_PORTA_AHB_PUR_R    {() => (0x40058510 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_PDR_R    {() => (0x40058514 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_SLR_R    {() => (0x40058518 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DEN_R    {() => (0x4005851C as *mut u32);}
macro_rules! GPIO_PORTA_AHB_LOCK_R   {() => (0x40058520 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_CR_R     {() => (0x40058524 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_AMSEL_R  {() => (0x40058528 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_PCTL_R   {() => (0x4005852C as *mut u32);}
macro_rules! GPIO_PORTA_AHB_ADCCTL_R {() => (0x40058530 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_DMACTL_R {() => (0x40058534 as *mut u32);}
macro_rules! GPIO_PORTA_AHB_SI_R     {() => (0x40058538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTB AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTB_AHB_DATA_BITS_R {() => (0x40059000 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DATA_R   {() => (0x400593FC as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DIR_R    {() => (0x40059400 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_IS_R     {() => (0x40059404 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_IBE_R    {() => (0x40059408 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_IEV_R    {() => (0x4005940C as *mut u32);}
macro_rules! GPIO_PORTB_AHB_IM_R     {() => (0x40059410 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_RIS_R    {() => (0x40059414 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_MIS_R    {() => (0x40059418 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_ICR_R    {() => (0x4005941C as *mut u32);}
macro_rules! GPIO_PORTB_AHB_AFSEL_R  {() => (0x40059420 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DR2R_R   {() => (0x40059500 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DR4R_R   {() => (0x40059504 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DR8R_R   {() => (0x40059508 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_ODR_R    {() => (0x4005950C as *mut u32);}
macro_rules! GPIO_PORTB_AHB_PUR_R    {() => (0x40059510 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_PDR_R    {() => (0x40059514 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_SLR_R    {() => (0x40059518 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DEN_R    {() => (0x4005951C as *mut u32);}
macro_rules! GPIO_PORTB_AHB_LOCK_R   {() => (0x40059520 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_CR_R     {() => (0x40059524 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_AMSEL_R  {() => (0x40059528 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_PCTL_R   {() => (0x4005952C as *mut u32);}
macro_rules! GPIO_PORTB_AHB_ADCCTL_R {() => (0x40059530 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_DMACTL_R {() => (0x40059534 as *mut u32);}
macro_rules! GPIO_PORTB_AHB_SI_R     {() => (0x40059538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTC AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTC_AHB_DATA_BITS_R {() => (0x4005A000 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DATA_R   {() => (0x4005A3FC as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DIR_R    {() => (0x4005A400 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_IS_R     {() => (0x4005A404 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_IBE_R    {() => (0x4005A408 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_IEV_R    {() => (0x4005A40C as *mut u32);}
macro_rules! GPIO_PORTC_AHB_IM_R     {() => (0x4005A410 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_RIS_R    {() => (0x4005A414 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_MIS_R    {() => (0x4005A418 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_ICR_R    {() => (0x4005A41C as *mut u32);}
macro_rules! GPIO_PORTC_AHB_AFSEL_R  {() => (0x4005A420 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DR2R_R   {() => (0x4005A500 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DR4R_R   {() => (0x4005A504 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DR8R_R   {() => (0x4005A508 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_ODR_R    {() => (0x4005A50C as *mut u32);}
macro_rules! GPIO_PORTC_AHB_PUR_R    {() => (0x4005A510 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_PDR_R    {() => (0x4005A514 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_SLR_R    {() => (0x4005A518 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DEN_R    {() => (0x4005A51C as *mut u32);}
macro_rules! GPIO_PORTC_AHB_LOCK_R   {() => (0x4005A520 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_CR_R     {() => (0x4005A524 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_AMSEL_R  {() => (0x4005A528 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_PCTL_R   {() => (0x4005A52C as *mut u32);}
macro_rules! GPIO_PORTC_AHB_ADCCTL_R {() => (0x4005A530 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_DMACTL_R {() => (0x4005A534 as *mut u32);}
macro_rules! GPIO_PORTC_AHB_SI_R     {() => (0x4005A538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTD AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTD_AHB_DATA_BITS_R {() => (0x4005B000 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DATA_R   {() => (0x4005B3FC as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DIR_R    {() => (0x4005B400 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_IS_R     {() => (0x4005B404 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_IBE_R    {() => (0x4005B408 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_IEV_R    {() => (0x4005B40C as *mut u32);}
macro_rules! GPIO_PORTD_AHB_IM_R     {() => (0x4005B410 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_RIS_R    {() => (0x4005B414 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_MIS_R    {() => (0x4005B418 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_ICR_R    {() => (0x4005B41C as *mut u32);}
macro_rules! GPIO_PORTD_AHB_AFSEL_R  {() => (0x4005B420 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DR2R_R   {() => (0x4005B500 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DR4R_R   {() => (0x4005B504 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DR8R_R   {() => (0x4005B508 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_ODR_R    {() => (0x4005B50C as *mut u32);}
macro_rules! GPIO_PORTD_AHB_PUR_R    {() => (0x4005B510 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_PDR_R    {() => (0x4005B514 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_SLR_R    {() => (0x4005B518 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DEN_R    {() => (0x4005B51C as *mut u32);}
macro_rules! GPIO_PORTD_AHB_LOCK_R   {() => (0x4005B520 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_CR_R     {() => (0x4005B524 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_AMSEL_R  {() => (0x4005B528 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_PCTL_R   {() => (0x4005B52C as *mut u32);}
macro_rules! GPIO_PORTD_AHB_ADCCTL_R {() => (0x4005B530 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_DMACTL_R {() => (0x4005B534 as *mut u32);}
macro_rules! GPIO_PORTD_AHB_SI_R     {() => (0x4005B538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTE AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTE_AHB_DATA_BITS_R {() => (0x4005C000 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DATA_R   {() => (0x4005C3FC as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DIR_R    {() => (0x4005C400 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_IS_R     {() => (0x4005C404 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_IBE_R    {() => (0x4005C408 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_IEV_R    {() => (0x4005C40C as *mut u32);}
macro_rules! GPIO_PORTE_AHB_IM_R     {() => (0x4005C410 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_RIS_R    {() => (0x4005C414 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_MIS_R    {() => (0x4005C418 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_ICR_R    {() => (0x4005C41C as *mut u32);}
macro_rules! GPIO_PORTE_AHB_AFSEL_R  {() => (0x4005C420 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DR2R_R   {() => (0x4005C500 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DR4R_R   {() => (0x4005C504 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DR8R_R   {() => (0x4005C508 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_ODR_R    {() => (0x4005C50C as *mut u32);}
macro_rules! GPIO_PORTE_AHB_PUR_R    {() => (0x4005C510 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_PDR_R    {() => (0x4005C514 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_SLR_R    {() => (0x4005C518 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DEN_R    {() => (0x4005C51C as *mut u32);}
macro_rules! GPIO_PORTE_AHB_LOCK_R   {() => (0x4005C520 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_CR_R     {() => (0x4005C524 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_AMSEL_R  {() => (0x4005C528 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_PCTL_R   {() => (0x4005C52C as *mut u32);}
macro_rules! GPIO_PORTE_AHB_ADCCTL_R {() => (0x4005C530 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_DMACTL_R {() => (0x4005C534 as *mut u32);}
macro_rules! GPIO_PORTE_AHB_SI_R     {() => (0x4005C538 as *mut u32);}

// *****************************************************************************
//
// GPIO registers (PORTF AHB)
//
// *****************************************************************************
macro_rules! GPIO_PORTF_AHB_DATA_BITS_R {() => (0x4005D000 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DATA_R   {() => (0x4005D3FC as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DIR_R    {() => (0x4005D400 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_IS_R     {() => (0x4005D404 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_IBE_R    {() => (0x4005D408 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_IEV_R    {() => (0x4005D40C as *mut u32);}
macro_rules! GPIO_PORTF_AHB_IM_R     {() => (0x4005D410 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_RIS_R    {() => (0x4005D414 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_MIS_R    {() => (0x4005D418 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_ICR_R    {() => (0x4005D41C as *mut u32);}
macro_rules! GPIO_PORTF_AHB_AFSEL_R  {() => (0x4005D420 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DR2R_R   {() => (0x4005D500 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DR4R_R   {() => (0x4005D504 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DR8R_R   {() => (0x4005D508 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_ODR_R    {() => (0x4005D50C as *mut u32);}
macro_rules! GPIO_PORTF_AHB_PUR_R    {() => (0x4005D510 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_PDR_R    {() => (0x4005D514 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_SLR_R    {() => (0x4005D518 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DEN_R    {() => (0x4005D51C as *mut u32);}
macro_rules! GPIO_PORTF_AHB_LOCK_R   {() => (0x4005D520 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_CR_R     {() => (0x4005D524 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_AMSEL_R  {() => (0x4005D528 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_PCTL_R   {() => (0x4005D52C as *mut u32);}
macro_rules! GPIO_PORTF_AHB_ADCCTL_R {() => (0x4005D530 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_DMACTL_R {() => (0x4005D534 as *mut u32);}
macro_rules! GPIO_PORTF_AHB_SI_R     {() => (0x4005D538 as *mut u32);}

// *****************************************************************************
//
// EEPROM registers (EEPROM)
//
// *****************************************************************************
macro_rules! EEPROM_EESIZE_R         {() => (0x400AF000 as *mut u32);}
macro_rules! EEPROM_EEBLOCK_R        {() => (0x400AF004 as *mut u32);}
macro_rules! EEPROM_EEOFFSET_R       {() => (0x400AF008 as *mut u32);}
macro_rules! EEPROM_EERDWR_R         {() => (0x400AF010 as *mut u32);}
macro_rules! EEPROM_EERDWRINC_R      {() => (0x400AF014 as *mut u32);}
macro_rules! EEPROM_EEDONE_R         {() => (0x400AF018 as *mut u32);}
macro_rules! EEPROM_EESUPP_R         {() => (0x400AF01C as *mut u32);}
macro_rules! EEPROM_EEUNLOCK_R       {() => (0x400AF020 as *mut u32);}
macro_rules! EEPROM_EEPROT_R         {() => (0x400AF030 as *mut u32);}
macro_rules! EEPROM_EEPASS0_R        {() => (0x400AF034 as *mut u32);}
macro_rules! EEPROM_EEPASS1_R        {() => (0x400AF038 as *mut u32);}
macro_rules! EEPROM_EEPASS2_R        {() => (0x400AF03C as *mut u32);}
macro_rules! EEPROM_EEINT_R          {() => (0x400AF040 as *mut u32);}
macro_rules! EEPROM_EEHIDE_R         {() => (0x400AF050 as *mut u32);}
macro_rules! EEPROM_EEDBGME_R        {() => (0x400AF080 as *mut u32);}
macro_rules! EEPROM_PP_R             {() => (0x400AFFC0 as *mut u32);}

// *****************************************************************************
//
// System Exception Module registers (SYSEXC)
//
// *****************************************************************************
macro_rules! SYSEXC_RIS_R            {() => (0x400F9000 as *mut u32);}
macro_rules! SYSEXC_IM_R             {() => (0x400F9004 as *mut u32);}
macro_rules! SYSEXC_MIS_R            {() => (0x400F9008 as *mut u32);}
macro_rules! SYSEXC_IC_R             {() => (0x400F900C as *mut u32);}

// *****************************************************************************
//
// Hibernation module registers (HIB)
//
// *****************************************************************************
macro_rules! HIB_RTCC_R              {() => (0x400FC000 as *mut u32);}
macro_rules! HIB_RTCM0_R             {() => (0x400FC004 as *mut u32);}
macro_rules! HIB_RTCLD_R             {() => (0x400FC00C as *mut u32);}
macro_rules! HIB_CTL_R               {() => (0x400FC010 as *mut u32);}
macro_rules! HIB_IM_R                {() => (0x400FC014 as *mut u32);}
macro_rules! HIB_RIS_R               {() => (0x400FC018 as *mut u32);}
macro_rules! HIB_MIS_R               {() => (0x400FC01C as *mut u32);}
macro_rules! HIB_IC_R                {() => (0x400FC020 as *mut u32);}
macro_rules! HIB_RTCT_R              {() => (0x400FC024 as *mut u32);}
macro_rules! HIB_RTCSS_R             {() => (0x400FC028 as *mut u32);}
macro_rules! HIB_DATA_R              {() => (0x400FC030 as *mut u32);}

// *****************************************************************************
//
// FLASH registers (FLASH CTRL)
//
// *****************************************************************************
macro_rules! FLASH_FMA_R             {() => (0x400FD000 as *mut u32);}
macro_rules! FLASH_FMD_R             {() => (0x400FD004 as *mut u32);}
macro_rules! FLASH_FMC_R             {() => (0x400FD008 as *mut u32);}
macro_rules! FLASH_FCRIS_R           {() => (0x400FD00C as *mut u32);}
macro_rules! FLASH_FCIM_R            {() => (0x400FD010 as *mut u32);}
macro_rules! FLASH_FCMISC_R          {() => (0x400FD014 as *mut u32);}
macro_rules! FLASH_FMC2_R            {() => (0x400FD020 as *mut u32);}
macro_rules! FLASH_FWBVAL_R          {() => (0x400FD030 as *mut u32);}
macro_rules! FLASH_FWBN_R            {() => (0x400FD100 as *mut u32);}
macro_rules! FLASH_FSIZE_R           {() => (0x400FDFC0 as *mut u32);}
macro_rules! FLASH_SSIZE_R           {() => (0x400FDFC4 as *mut u32);}
macro_rules! FLASH_ROMSWMAP_R        {() => (0x400FDFCC as *mut u32);}
macro_rules! FLASH_RMCTL_R           {() => (0x400FE0F0 as *mut u32);}
macro_rules! FLASH_BOOTCFG_R         {() => (0x400FE1D0 as *mut u32);}
macro_rules! FLASH_USERREG0_R        {() => (0x400FE1E0 as *mut u32);}
macro_rules! FLASH_USERREG1_R        {() => (0x400FE1E4 as *mut u32);}
macro_rules! FLASH_USERREG2_R        {() => (0x400FE1E8 as *mut u32);}
macro_rules! FLASH_USERREG3_R        {() => (0x400FE1EC as *mut u32);}
macro_rules! FLASH_FMPRE0_R          {() => (0x400FE200 as *mut u32);}
macro_rules! FLASH_FMPRE1_R          {() => (0x400FE204 as *mut u32);}
macro_rules! FLASH_FMPRE2_R          {() => (0x400FE208 as *mut u32);}
macro_rules! FLASH_FMPRE3_R          {() => (0x400FE20C as *mut u32);}
macro_rules! FLASH_FMPPE0_R          {() => (0x400FE400 as *mut u32);}
macro_rules! FLASH_FMPPE1_R          {() => (0x400FE404 as *mut u32);}
macro_rules! FLASH_FMPPE2_R          {() => (0x400FE408 as *mut u32);}
macro_rules! FLASH_FMPPE3_R          {() => (0x400FE40C as *mut u32);}

// *****************************************************************************
//
// System Control registers (SYSCTL)
//
// *****************************************************************************
macro_rules! SYSCTL_DID0_R           {() => (0x400FE000 as *mut u32);}
macro_rules! SYSCTL_DID1_R           {() => (0x400FE004 as *mut u32);}
macro_rules! SYSCTL_DC0_R            {() => (0x400FE008 as *mut u32);}
macro_rules! SYSCTL_DC1_R            {() => (0x400FE010 as *mut u32);}
macro_rules! SYSCTL_DC2_R            {() => (0x400FE014 as *mut u32);}
macro_rules! SYSCTL_DC3_R            {() => (0x400FE018 as *mut u32);}
macro_rules! SYSCTL_DC4_R            {() => (0x400FE01C as *mut u32);}
macro_rules! SYSCTL_DC5_R            {() => (0x400FE020 as *mut u32);}
macro_rules! SYSCTL_DC6_R            {() => (0x400FE024 as *mut u32);}
macro_rules! SYSCTL_DC7_R            {() => (0x400FE028 as *mut u32);}
macro_rules! SYSCTL_DC8_R            {() => (0x400FE02C as *mut u32);}
macro_rules! SYSCTL_PBORCTL_R        {() => (0x400FE030 as *mut u32);}
macro_rules! SYSCTL_SRCR0_R          {() => (0x400FE040 as *mut u32);}
macro_rules! SYSCTL_SRCR1_R          {() => (0x400FE044 as *mut u32);}
macro_rules! SYSCTL_SRCR2_R          {() => (0x400FE048 as *mut u32);}
macro_rules! SYSCTL_RIS_R            {() => (0x400FE050 as *mut u32);}
macro_rules! SYSCTL_IMC_R            {() => (0x400FE054 as *mut u32);}
macro_rules! SYSCTL_MISC_R           {() => (0x400FE058 as *mut u32);}
macro_rules! SYSCTL_RESC_R           {() => (0x400FE05C as *mut u32);}
macro_rules! SYSCTL_RCC_R            {() => (0x400FE060 as *mut u32);}
macro_rules! SYSCTL_GPIOHBCTL_R      {() => (0x400FE06C as *mut u32);}
macro_rules! SYSCTL_RCC2_R           {() => (0x400FE070 as *mut u32);}
macro_rules! SYSCTL_MOSCCTL_R        {() => (0x400FE07C as *mut u32);}
macro_rules! SYSCTL_RCGC0_R          {() => (0x400FE100 as *mut u32);}
macro_rules! SYSCTL_RCGC1_R          {() => (0x400FE104 as *mut u32);}
macro_rules! SYSCTL_RCGC2_R          {() => (0x400FE108 as *mut u32);}
macro_rules! SYSCTL_SCGC0_R          {() => (0x400FE110 as *mut u32);}
macro_rules! SYSCTL_SCGC1_R          {() => (0x400FE114 as *mut u32);}
macro_rules! SYSCTL_SCGC2_R          {() => (0x400FE118 as *mut u32);}
macro_rules! SYSCTL_DCGC0_R          {() => (0x400FE120 as *mut u32);}
macro_rules! SYSCTL_DCGC1_R          {() => (0x400FE124 as *mut u32);}
macro_rules! SYSCTL_DCGC2_R          {() => (0x400FE128 as *mut u32);}
macro_rules! SYSCTL_DSLPCLKCFG_R     {() => (0x400FE144 as *mut u32);}
macro_rules! SYSCTL_SYSPROP_R        {() => (0x400FE14C as *mut u32);}
macro_rules! SYSCTL_PIOSCCAL_R       {() => (0x400FE150 as *mut u32);}
macro_rules! SYSCTL_PIOSCSTAT_R      {() => (0x400FE154 as *mut u32);}
macro_rules! SYSCTL_PLLFREQ0_R       {() => (0x400FE160 as *mut u32);}
macro_rules! SYSCTL_PLLFREQ1_R       {() => (0x400FE164 as *mut u32);}
macro_rules! SYSCTL_PLLSTAT_R        {() => (0x400FE168 as *mut u32);}
macro_rules! SYSCTL_DC9_R            {() => (0x400FE190 as *mut u32);}
macro_rules! SYSCTL_NVMSTAT_R        {() => (0x400FE1A0 as *mut u32);}
macro_rules! SYSCTL_PPWD_R           {() => (0x400FE300 as *mut u32);}
macro_rules! SYSCTL_PPTIMER_R        {() => (0x400FE304 as *mut u32);}
macro_rules! SYSCTL_PPGPIO_R         {() => (0x400FE308 as *mut u32);}
macro_rules! SYSCTL_PPDMA_R          {() => (0x400FE30C as *mut u32);}
macro_rules! SYSCTL_PPHIB_R          {() => (0x400FE314 as *mut u32);}
macro_rules! SYSCTL_PPUART_R         {() => (0x400FE318 as *mut u32);}
macro_rules! SYSCTL_PPSSI_R          {() => (0x400FE31C as *mut u32);}
macro_rules! SYSCTL_PPI2C_R          {() => (0x400FE320 as *mut u32);}
macro_rules! SYSCTL_PPUSB_R          {() => (0x400FE328 as *mut u32);}
macro_rules! SYSCTL_PPCAN_R          {() => (0x400FE334 as *mut u32);}
macro_rules! SYSCTL_PPADC_R          {() => (0x400FE338 as *mut u32);}
macro_rules! SYSCTL_PPACMP_R         {() => (0x400FE33C as *mut u32);}
macro_rules! SYSCTL_PPPWM_R          {() => (0x400FE340 as *mut u32);}
macro_rules! SYSCTL_PPQEI_R          {() => (0x400FE344 as *mut u32);}
macro_rules! SYSCTL_PPEEPROM_R       {() => (0x400FE358 as *mut u32);}
macro_rules! SYSCTL_PPWTIMER_R       {() => (0x400FE35C as *mut u32);}
macro_rules! SYSCTL_SRWD_R           {() => (0x400FE500 as *mut u32);}
macro_rules! SYSCTL_SRTIMER_R        {() => (0x400FE504 as *mut u32);}
macro_rules! SYSCTL_SRGPIO_R         {() => (0x400FE508 as *mut u32);}
macro_rules! SYSCTL_SRDMA_R          {() => (0x400FE50C as *mut u32);}
macro_rules! SYSCTL_SRHIB_R          {() => (0x400FE514 as *mut u32);}
macro_rules! SYSCTL_SRUART_R         {() => (0x400FE518 as *mut u32);}
macro_rules! SYSCTL_SRSSI_R          {() => (0x400FE51C as *mut u32);}
macro_rules! SYSCTL_SRI2C_R          {() => (0x400FE520 as *mut u32);}
macro_rules! SYSCTL_SRUSB_R          {() => (0x400FE528 as *mut u32);}
macro_rules! SYSCTL_SRCAN_R          {() => (0x400FE534 as *mut u32);}
macro_rules! SYSCTL_SRADC_R          {() => (0x400FE538 as *mut u32);}
macro_rules! SYSCTL_SRACMP_R         {() => (0x400FE53C as *mut u32);}
macro_rules! SYSCTL_SREEPROM_R       {() => (0x400FE558 as *mut u32);}
macro_rules! SYSCTL_SRWTIMER_R       {() => (0x400FE55C as *mut u32);}
macro_rules! SYSCTL_RCGCWD_R         {() => (0x400FE600 as *mut u32);}
macro_rules! SYSCTL_RCGCTIMER_R      {() => (0x400FE604 as *mut u32);}
macro_rules! SYSCTL_RCGCGPIO_R       {() => (0x400FE608 as *mut u32);}
macro_rules! SYSCTL_RCGCDMA_R        {() => (0x400FE60C as *mut u32);}
macro_rules! SYSCTL_RCGCHIB_R        {() => (0x400FE614 as *mut u32);}
macro_rules! SYSCTL_RCGCUART_R       {() => (0x400FE618 as *mut u32);}
macro_rules! SYSCTL_RCGCSSI_R        {() => (0x400FE61C as *mut u32);}
macro_rules! SYSCTL_RCGCI2C_R        {() => (0x400FE620 as *mut u32);}
macro_rules! SYSCTL_RCGCUSB_R        {() => (0x400FE628 as *mut u32);}
macro_rules! SYSCTL_RCGCCAN_R        {() => (0x400FE634 as *mut u32);}
macro_rules! SYSCTL_RCGCADC_R        {() => (0x400FE638 as *mut u32);}
macro_rules! SYSCTL_RCGCACMP_R       {() => (0x400FE63C as *mut u32);}
macro_rules! SYSCTL_RCGCEEPROM_R     {() => (0x400FE658 as *mut u32);}
macro_rules! SYSCTL_RCGCWTIMER_R     {() => (0x400FE65C as *mut u32);}
macro_rules! SYSCTL_SCGCWD_R         {() => (0x400FE700 as *mut u32);}
macro_rules! SYSCTL_SCGCTIMER_R      {() => (0x400FE704 as *mut u32);}
macro_rules! SYSCTL_SCGCGPIO_R       {() => (0x400FE708 as *mut u32);}
macro_rules! SYSCTL_SCGCDMA_R        {() => (0x400FE70C as *mut u32);}
macro_rules! SYSCTL_SCGCHIB_R        {() => (0x400FE714 as *mut u32);}
macro_rules! SYSCTL_SCGCUART_R       {() => (0x400FE718 as *mut u32);}
macro_rules! SYSCTL_SCGCSSI_R        {() => (0x400FE71C as *mut u32);}
macro_rules! SYSCTL_SCGCI2C_R        {() => (0x400FE720 as *mut u32);}
macro_rules! SYSCTL_SCGCUSB_R        {() => (0x400FE728 as *mut u32);}
macro_rules! SYSCTL_SCGCCAN_R        {() => (0x400FE734 as *mut u32);}
macro_rules! SYSCTL_SCGCADC_R        {() => (0x400FE738 as *mut u32);}
macro_rules! SYSCTL_SCGCACMP_R       {() => (0x400FE73C as *mut u32);}
macro_rules! SYSCTL_SCGCEEPROM_R     {() => (0x400FE758 as *mut u32);}
macro_rules! SYSCTL_SCGCWTIMER_R     {() => (0x400FE75C as *mut u32);}
macro_rules! SYSCTL_DCGCWD_R         {() => (0x400FE800 as *mut u32);}
macro_rules! SYSCTL_DCGCTIMER_R      {() => (0x400FE804 as *mut u32);}
macro_rules! SYSCTL_DCGCGPIO_R       {() => (0x400FE808 as *mut u32);}
macro_rules! SYSCTL_DCGCDMA_R        {() => (0x400FE80C as *mut u32);}
macro_rules! SYSCTL_DCGCHIB_R        {() => (0x400FE814 as *mut u32);}
macro_rules! SYSCTL_DCGCUART_R       {() => (0x400FE818 as *mut u32);}
macro_rules! SYSCTL_DCGCSSI_R        {() => (0x400FE81C as *mut u32);}
macro_rules! SYSCTL_DCGCI2C_R        {() => (0x400FE820 as *mut u32);}
macro_rules! SYSCTL_DCGCUSB_R        {() => (0x400FE828 as *mut u32);}
macro_rules! SYSCTL_DCGCCAN_R        {() => (0x400FE834 as *mut u32);}
macro_rules! SYSCTL_DCGCADC_R        {() => (0x400FE838 as *mut u32);}
macro_rules! SYSCTL_DCGCACMP_R       {() => (0x400FE83C as *mut u32);}
macro_rules! SYSCTL_DCGCEEPROM_R     {() => (0x400FE858 as *mut u32);}
macro_rules! SYSCTL_DCGCWTIMER_R     {() => (0x400FE85C as *mut u32);}
macro_rules! SYSCTL_PCWD_R           {() => (0x400FE900 as *mut u32);}
macro_rules! SYSCTL_PCTIMER_R        {() => (0x400FE904 as *mut u32);}
macro_rules! SYSCTL_PCGPIO_R         {() => (0x400FE908 as *mut u32);}
macro_rules! SYSCTL_PCDMA_R          {() => (0x400FE90C as *mut u32);}
macro_rules! SYSCTL_PCHIB_R          {() => (0x400FE914 as *mut u32);}
macro_rules! SYSCTL_PCUART_R         {() => (0x400FE918 as *mut u32);}
macro_rules! SYSCTL_PCSSI_R          {() => (0x400FE91C as *mut u32);}
macro_rules! SYSCTL_PCI2C_R          {() => (0x400FE920 as *mut u32);}
macro_rules! SYSCTL_PCUSB_R          {() => (0x400FE928 as *mut u32);}
macro_rules! SYSCTL_PCCAN_R          {() => (0x400FE934 as *mut u32);}
macro_rules! SYSCTL_PCADC_R          {() => (0x400FE938 as *mut u32);}
macro_rules! SYSCTL_PCACMP_R         {() => (0x400FE93C as *mut u32);}
macro_rules! SYSCTL_PCEEPROM_R       {() => (0x400FE958 as *mut u32);}
macro_rules! SYSCTL_PCWTIMER_R       {() => (0x400FE95C as *mut u32);}
macro_rules! SYSCTL_PRWD_R           {() => (0x400FEA00 as *mut u32);}
macro_rules! SYSCTL_PRTIMER_R        {() => (0x400FEA04 as *mut u32);}
macro_rules! SYSCTL_PRGPIO_R         {() => (0x400FEA08 as *mut u32);}
macro_rules! SYSCTL_PRDMA_R          {() => (0x400FEA0C as *mut u32);}
macro_rules! SYSCTL_PRHIB_R          {() => (0x400FEA14 as *mut u32);}
macro_rules! SYSCTL_PRUART_R         {() => (0x400FEA18 as *mut u32);}
macro_rules! SYSCTL_PRSSI_R          {() => (0x400FEA1C as *mut u32);}
macro_rules! SYSCTL_PRI2C_R          {() => (0x400FEA20 as *mut u32);}
macro_rules! SYSCTL_PRUSB_R          {() => (0x400FEA28 as *mut u32);}
macro_rules! SYSCTL_PRCAN_R          {() => (0x400FEA34 as *mut u32);}
macro_rules! SYSCTL_PRADC_R          {() => (0x400FEA38 as *mut u32);}
macro_rules! SYSCTL_PRACMP_R         {() => (0x400FEA3C as *mut u32);}
macro_rules! SYSCTL_PREEPROM_R       {() => (0x400FEA58 as *mut u32);}
macro_rules! SYSCTL_PRWTIMER_R       {() => (0x400FEA5C as *mut u32);}

// *****************************************************************************
//
// Micro Direct Memory Access registers (UDMA)
//
// *****************************************************************************
macro_rules! UDMA_STAT_R             {() => (0x400FF000 as *mut u32);}
macro_rules! UDMA_CFG_R              {() => (0x400FF004 as *mut u32);}
macro_rules! UDMA_CTLBASE_R          {() => (0x400FF008 as *mut u32);}
macro_rules! UDMA_ALTBASE_R          {() => (0x400FF00C as *mut u32);}
macro_rules! UDMA_WAITSTAT_R         {() => (0x400FF010 as *mut u32);}
macro_rules! UDMA_SWREQ_R            {() => (0x400FF014 as *mut u32);}
macro_rules! UDMA_USEBURSTSET_R      {() => (0x400FF018 as *mut u32);}
macro_rules! UDMA_USEBURSTCLR_R      {() => (0x400FF01C as *mut u32);}
macro_rules! UDMA_REQMASKSET_R       {() => (0x400FF020 as *mut u32);}
macro_rules! UDMA_REQMASKCLR_R       {() => (0x400FF024 as *mut u32);}
macro_rules! UDMA_ENASET_R           {() => (0x400FF028 as *mut u32);}
macro_rules! UDMA_ENACLR_R           {() => (0x400FF02C as *mut u32);}
macro_rules! UDMA_ALTSET_R           {() => (0x400FF030 as *mut u32);}
macro_rules! UDMA_ALTCLR_R           {() => (0x400FF034 as *mut u32);}
macro_rules! UDMA_PRIOSET_R          {() => (0x400FF038 as *mut u32);}
macro_rules! UDMA_PRIOCLR_R          {() => (0x400FF03C as *mut u32);}
macro_rules! UDMA_ERRCLR_R           {() => (0x400FF04C as *mut u32);}
macro_rules! UDMA_CHASGN_R           {() => (0x400FF500 as *mut u32);}
macro_rules! UDMA_CHIS_R             {() => (0x400FF504 as *mut u32);}
macro_rules! UDMA_CHMAP0_R           {() => (0x400FF510 as *mut u32);}
macro_rules! UDMA_CHMAP1_R           {() => (0x400FF514 as *mut u32);}
macro_rules! UDMA_CHMAP2_R           {() => (0x400FF518 as *mut u32);}
macro_rules! UDMA_CHMAP3_R           {() => (0x400FF51C as *mut u32);}

// *****************************************************************************
//
// Micro Direct Memory Access (uDMA) offsets (UDMA)
//
// *****************************************************************************
const UDMA_SRCENDP: u32 = 0x00000000; // DMA Channel Source Address End Pointer
const UDMA_DSTENDP: u32 = 0x00000004; // DMA Channel Destination Address End Pointer
const UDMA_CHCTL: u32 = 0x00000008; // DMA Channel Control Word

// *****************************************************************************
//
// NVIC registers (NVIC)
//
// *****************************************************************************
macro_rules! NVIC_INT_TYPE_R         {() => (0xE000E004 as *mut u32);}
macro_rules! NVIC_ACTLR_R            {() => (0xE000E008 as *mut u32);}
macro_rules! NVIC_ST_CTRL_R          {() => (0xE000E010 as *mut u32);}
macro_rules! NVIC_ST_RELOAD_R        {() => (0xE000E014 as *mut u32);}
macro_rules! NVIC_ST_CURRENT_R       {() => (0xE000E018 as *mut u32);}
macro_rules! NVIC_ST_CAL_R           {() => (0xE000E01C as *mut u32);}
macro_rules! NVIC_EN0_R              {() => (0xE000E100 as *mut u32);}
macro_rules! NVIC_EN1_R              {() => (0xE000E104 as *mut u32);}
macro_rules! NVIC_EN2_R              {() => (0xE000E108 as *mut u32);}
macro_rules! NVIC_EN3_R              {() => (0xE000E10C as *mut u32);}
macro_rules! NVIC_EN4_R              {() => (0xE000E110 as *mut u32);}
macro_rules! NVIC_DIS0_R             {() => (0xE000E180 as *mut u32);}
macro_rules! NVIC_DIS1_R             {() => (0xE000E184 as *mut u32);}
macro_rules! NVIC_DIS2_R             {() => (0xE000E188 as *mut u32);}
macro_rules! NVIC_DIS3_R             {() => (0xE000E18C as *mut u32);}
macro_rules! NVIC_DIS4_R             {() => (0xE000E190 as *mut u32);}
macro_rules! NVIC_PEND0_R            {() => (0xE000E200 as *mut u32);}
macro_rules! NVIC_PEND1_R            {() => (0xE000E204 as *mut u32);}
macro_rules! NVIC_PEND2_R            {() => (0xE000E208 as *mut u32);}
macro_rules! NVIC_PEND3_R            {() => (0xE000E20C as *mut u32);}
macro_rules! NVIC_PEND4_R            {() => (0xE000E210 as *mut u32);}
macro_rules! NVIC_UNPEND0_R          {() => (0xE000E280 as *mut u32);}
macro_rules! NVIC_UNPEND1_R          {() => (0xE000E284 as *mut u32);}
macro_rules! NVIC_UNPEND2_R          {() => (0xE000E288 as *mut u32);}
macro_rules! NVIC_UNPEND3_R          {() => (0xE000E28C as *mut u32);}
macro_rules! NVIC_UNPEND4_R          {() => (0xE000E290 as *mut u32);}
macro_rules! NVIC_ACTIVE0_R          {() => (0xE000E300 as *mut u32);}
macro_rules! NVIC_ACTIVE1_R          {() => (0xE000E304 as *mut u32);}
macro_rules! NVIC_ACTIVE2_R          {() => (0xE000E308 as *mut u32);}
macro_rules! NVIC_ACTIVE3_R          {() => (0xE000E30C as *mut u32);}
macro_rules! NVIC_ACTIVE4_R          {() => (0xE000E310 as *mut u32);}
macro_rules! NVIC_PRI0_R             {() => (0xE000E400 as *mut u32);}
macro_rules! NVIC_PRI1_R             {() => (0xE000E404 as *mut u32);}
macro_rules! NVIC_PRI2_R             {() => (0xE000E408 as *mut u32);}
macro_rules! NVIC_PRI3_R             {() => (0xE000E40C as *mut u32);}
macro_rules! NVIC_PRI4_R             {() => (0xE000E410 as *mut u32);}
macro_rules! NVIC_PRI5_R             {() => (0xE000E414 as *mut u32);}
macro_rules! NVIC_PRI6_R             {() => (0xE000E418 as *mut u32);}
macro_rules! NVIC_PRI7_R             {() => (0xE000E41C as *mut u32);}
macro_rules! NVIC_PRI8_R             {() => (0xE000E420 as *mut u32);}
macro_rules! NVIC_PRI9_R             {() => (0xE000E424 as *mut u32);}
macro_rules! NVIC_PRI10_R            {() => (0xE000E428 as *mut u32);}
macro_rules! NVIC_PRI11_R            {() => (0xE000E42C as *mut u32);}
macro_rules! NVIC_PRI12_R            {() => (0xE000E430 as *mut u32);}
macro_rules! NVIC_PRI13_R            {() => (0xE000E434 as *mut u32);}
macro_rules! NVIC_PRI14_R            {() => (0xE000E438 as *mut u32);}
macro_rules! NVIC_PRI15_R            {() => (0xE000E43C as *mut u32);}
macro_rules! NVIC_PRI16_R            {() => (0xE000E440 as *mut u32);}
macro_rules! NVIC_PRI17_R            {() => (0xE000E444 as *mut u32);}
macro_rules! NVIC_PRI18_R            {() => (0xE000E448 as *mut u32);}
macro_rules! NVIC_PRI19_R            {() => (0xE000E44C as *mut u32);}
macro_rules! NVIC_PRI20_R            {() => (0xE000E450 as *mut u32);}
macro_rules! NVIC_PRI21_R            {() => (0xE000E454 as *mut u32);}
macro_rules! NVIC_PRI22_R            {() => (0xE000E458 as *mut u32);}
macro_rules! NVIC_PRI23_R            {() => (0xE000E45C as *mut u32);}
macro_rules! NVIC_PRI24_R            {() => (0xE000E460 as *mut u32);}
macro_rules! NVIC_PRI25_R            {() => (0xE000E464 as *mut u32);}
macro_rules! NVIC_PRI26_R            {() => (0xE000E468 as *mut u32);}
macro_rules! NVIC_PRI27_R            {() => (0xE000E46C as *mut u32);}
macro_rules! NVIC_PRI28_R            {() => (0xE000E470 as *mut u32);}
macro_rules! NVIC_PRI29_R            {() => (0xE000E474 as *mut u32);}
macro_rules! NVIC_PRI30_R            {() => (0xE000E478 as *mut u32);}
macro_rules! NVIC_PRI31_R            {() => (0xE000E47C as *mut u32);}
macro_rules! NVIC_PRI32_R            {() => (0xE000E480 as *mut u32);}
macro_rules! NVIC_PRI33_R            {() => (0xE000E484 as *mut u32);}
macro_rules! NVIC_PRI34_R            {() => (0xE000E488 as *mut u32);}
macro_rules! NVIC_CPUID_R            {() => (0xE000ED00 as *mut u32);}
macro_rules! NVIC_INT_CTRL_R         {() => (0xE000ED04 as *mut u32);}
macro_rules! NVIC_VTABLE_R           {() => (0xE000ED08 as *mut u32);}
macro_rules! NVIC_APINT_R            {() => (0xE000ED0C as *mut u32);}
macro_rules! NVIC_SYS_CTRL_R         {() => (0xE000ED10 as *mut u32);}
macro_rules! NVIC_CFG_CTRL_R         {() => (0xE000ED14 as *mut u32);}
macro_rules! NVIC_SYS_PRI1_R         {() => (0xE000ED18 as *mut u32);}
macro_rules! NVIC_SYS_PRI2_R         {() => (0xE000ED1C as *mut u32);}
macro_rules! NVIC_SYS_PRI3_R         {() => (0xE000ED20 as *mut u32);}
macro_rules! NVIC_SYS_HND_CTRL_R     {() => (0xE000ED24 as *mut u32);}
macro_rules! NVIC_FAULT_STAT_R       {() => (0xE000ED28 as *mut u32);}
macro_rules! NVIC_HFAULT_STAT_R      {() => (0xE000ED2C as *mut u32);}
macro_rules! NVIC_DEBUG_STAT_R       {() => (0xE000ED30 as *mut u32);}
macro_rules! NVIC_MM_ADDR_R          {() => (0xE000ED34 as *mut u32);}
macro_rules! NVIC_FAULT_ADDR_R       {() => (0xE000ED38 as *mut u32);}
macro_rules! NVIC_CPAC_R             {() => (0xE000ED88 as *mut u32);}
macro_rules! NVIC_MPU_TYPE_R         {() => (0xE000ED90 as *mut u32);}
macro_rules! NVIC_MPU_CTRL_R         {() => (0xE000ED94 as *mut u32);}
macro_rules! NVIC_MPU_NUMBER_R       {() => (0xE000ED98 as *mut u32);}
macro_rules! NVIC_MPU_BASE_R         {() => (0xE000ED9C as *mut u32);}
macro_rules! NVIC_MPU_ATTR_R         {() => (0xE000EDA0 as *mut u32);}
macro_rules! NVIC_MPU_BASE1_R        {() => (0xE000EDA4 as *mut u32);}
macro_rules! NVIC_MPU_ATTR1_R        {() => (0xE000EDA8 as *mut u32);}
macro_rules! NVIC_MPU_BASE2_R        {() => (0xE000EDAC as *mut u32);}
macro_rules! NVIC_MPU_ATTR2_R        {() => (0xE000EDB0 as *mut u32);}
macro_rules! NVIC_MPU_BASE3_R        {() => (0xE000EDB4 as *mut u32);}
macro_rules! NVIC_MPU_ATTR3_R        {() => (0xE000EDB8 as *mut u32);}
macro_rules! NVIC_DBG_CTRL_R         {() => (0xE000EDF0 as *mut u32);}
macro_rules! NVIC_DBG_XFER_R         {() => (0xE000EDF4 as *mut u32);}
macro_rules! NVIC_DBG_DATA_R         {() => (0xE000EDF8 as *mut u32);}
macro_rules! NVIC_DBG_INT_R          {() => (0xE000EDFC as *mut u32);}
macro_rules! NVIC_SW_TRIG_R          {() => (0xE000EF00 as *mut u32);}
macro_rules! NVIC_FPCC_R             {() => (0xE000EF34 as *mut u32);}
macro_rules! NVIC_FPCA_R             {() => (0xE000EF38 as *mut u32);}
macro_rules! NVIC_FPDSC_R            {() => (0xE000EF3C as *mut u32);}

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_LOAD register.
//
// *****************************************************************************
const WDT_LOAD_M: u32 = 0xFFFFFFFF; // Watchdog Load Value
const WDT_LOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_VALUE register.
//
// *****************************************************************************
const WDT_VALUE_M: u32 = 0xFFFFFFFF; // Watchdog Value
const WDT_VALUE_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_CTL register.
//
// *****************************************************************************
const WDT_CTL_WRC: u32 = 0x80000000; // Write Complete
const WDT_CTL_INTTYPE: u32 = 0x00000004; // Watchdog Interrupt Type
const WDT_CTL_RESEN: u32 = 0x00000002; // Watchdog Reset Enable
const WDT_CTL_INTEN: u32 = 0x00000001; // Watchdog Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_ICR register.
//
// *****************************************************************************
const WDT_ICR_M: u32 = 0xFFFFFFFF; // Watchdog Interrupt Clear
const WDT_ICR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_RIS register.
//
// *****************************************************************************
const WDT_RIS_WDTRIS: u32 = 0x00000001; // Watchdog Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_MIS register.
//
// *****************************************************************************
const WDT_MIS_WDTMIS: u32 = 0x00000001; // Watchdog Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_TEST register.
//
// *****************************************************************************
const WDT_TEST_STALL: u32 = 0x00000100; // Watchdog Stall Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the WDT_O_LOCK register.
//
// *****************************************************************************
const WDT_LOCK_M: u32 = 0xFFFFFFFF; // Watchdog Lock
const WDT_LOCK_UNLOCKED: u32 = 0x00000000; // Unlocked
const WDT_LOCK_LOCKED: u32 = 0x00000001; // Locked

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_IM register.
//
// *****************************************************************************
const GPIO_IM_GPIO_M: u32 = 0x000000FF; // GPIO Interrupt Mask Enable
const GPIO_IM_GPIO_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_RIS register.
//
// *****************************************************************************
const GPIO_RIS_GPIO_M: u32 = 0x000000FF; // GPIO Interrupt Raw Status
const GPIO_RIS_GPIO_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_MIS register.
//
// *****************************************************************************
const GPIO_MIS_GPIO_M: u32 = 0x000000FF; // GPIO Masked Interrupt Status
const GPIO_MIS_GPIO_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_ICR register.
//
// *****************************************************************************
const GPIO_ICR_GPIO_M: u32 = 0x000000FF; // GPIO Interrupt Clear
const GPIO_ICR_GPIO_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_LOCK register.
//
// *****************************************************************************
const GPIO_LOCK_M: u32 = 0xFFFFFFFF; // GPIO Lock
const GPIO_LOCK_UNLOCKED: u32 = 0x00000000; // The GPIOCR register is unlocked and may be modified
const GPIO_LOCK_LOCKED: u32 = 0x00000001; // The GPIOCR register is locked and may not be modified
const GPIO_LOCK_KEY: u32 = 0x4C4F434B; // Unlocks the GPIO_CR register

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_O_SI register.
//
// *****************************************************************************
const GPIO_SI_SUM: u32 = 0x00000001; // Summary Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port A.
//
// *****************************************************************************
const GPIO_PCTL_PA7_M: u32 = 0xF0000000; // PA7 mask
const GPIO_PCTL_PA7_I2C1SDA: u32 = 0x30000000; // I2C1SDA on PA7
const GPIO_PCTL_PA6_M: u32 = 0x0F000000; // PA6 mask
const GPIO_PCTL_PA6_I2C1SCL: u32 = 0x03000000; // I2C1SCL on PA6
const GPIO_PCTL_PA5_M: u32 = 0x00F00000; // PA5 mask
const GPIO_PCTL_PA5_SSI0TX: u32 = 0x00200000; // SSI0TX on PA5
const GPIO_PCTL_PA4_M: u32 = 0x000F0000; // PA4 mask
const GPIO_PCTL_PA4_SSI0RX: u32 = 0x00020000; // SSI0RX on PA4
const GPIO_PCTL_PA3_M: u32 = 0x0000F000; // PA3 mask
const GPIO_PCTL_PA3_SSI0FSS: u32 = 0x00002000; // SSI0FSS on PA3
const GPIO_PCTL_PA2_M: u32 = 0x00000F00; // PA2 mask
const GPIO_PCTL_PA2_SSI0CLK: u32 = 0x00000200; // SSI0CLK on PA2
const GPIO_PCTL_PA1_M: u32 = 0x000000F0; // PA1 mask
const GPIO_PCTL_PA1_U0TX: u32 = 0x00000010; // U0TX on PA1
const GPIO_PCTL_PA0_M: u32 = 0x0000000F; // PA0 mask
const GPIO_PCTL_PA0_U0RX: u32 = 0x00000001; // U0RX on PA0

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port B.
//
// *****************************************************************************
const GPIO_PCTL_PB7_M: u32 = 0xF0000000; // PB7 mask
const GPIO_PCTL_PB7_SSI2TX: u32 = 0x20000000; // SSI2TX on PB7
const GPIO_PCTL_PB7_T0CCP1: u32 = 0x70000000; // T0CCP1 on PB7
const GPIO_PCTL_PB6_M: u32 = 0x0F000000; // PB6 mask
const GPIO_PCTL_PB6_SSI2RX: u32 = 0x02000000; // SSI2RX on PB6
const GPIO_PCTL_PB6_T0CCP0: u32 = 0x07000000; // T0CCP0 on PB6
const GPIO_PCTL_PB5_M: u32 = 0x00F00000; // PB5 mask
const GPIO_PCTL_PB5_SSI2FSS: u32 = 0x00200000; // SSI2FSS on PB5
const GPIO_PCTL_PB5_T1CCP1: u32 = 0x00700000; // T1CCP1 on PB5
const GPIO_PCTL_PB5_CAN0TX: u32 = 0x00800000; // CAN0TX on PB5
const GPIO_PCTL_PB4_M: u32 = 0x000F0000; // PB4 mask
const GPIO_PCTL_PB4_SSI2CLK: u32 = 0x00020000; // SSI2CLK on PB4
const GPIO_PCTL_PB4_T1CCP0: u32 = 0x00070000; // T1CCP0 on PB4
const GPIO_PCTL_PB4_CAN0RX: u32 = 0x00080000; // CAN0RX on PB4
const GPIO_PCTL_PB3_M: u32 = 0x0000F000; // PB3 mask
const GPIO_PCTL_PB3_I2C0SDA: u32 = 0x00003000; // I2C0SDA on PB3
const GPIO_PCTL_PB3_T3CCP1: u32 = 0x00007000; // T3CCP1 on PB3
const GPIO_PCTL_PB2_M: u32 = 0x00000F00; // PB2 mask
const GPIO_PCTL_PB2_I2C0SCL: u32 = 0x00000300; // I2C0SCL on PB2
const GPIO_PCTL_PB2_T3CCP0: u32 = 0x00000700; // T3CCP0 on PB2
const GPIO_PCTL_PB1_M: u32 = 0x000000F0; // PB1 mask
const GPIO_PCTL_PB1_U1TX: u32 = 0x00000010; // U1TX on PB1
const GPIO_PCTL_PB1_T2CCP1: u32 = 0x00000070; // T2CCP1 on PB1
const GPIO_PCTL_PB0_M: u32 = 0x0000000F; // PB0 mask
const GPIO_PCTL_PB0_U1RX: u32 = 0x00000001; // U1RX on PB0
const GPIO_PCTL_PB0_T2CCP0: u32 = 0x00000007; // T2CCP0 on PB0

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port C.
//
// *****************************************************************************
const GPIO_PCTL_PC7_M: u32 = 0xF0000000; // PC7 mask
const GPIO_PCTL_PC7_U3TX: u32 = 0x10000000; // U3TX on PC7
const GPIO_PCTL_PC7_WT1CCP1: u32 = 0x70000000; // WT1CCP1 on PC7
const GPIO_PCTL_PC6_M: u32 = 0x0F000000; // PC6 mask
const GPIO_PCTL_PC6_U3RX: u32 = 0x01000000; // U3RX on PC6
const GPIO_PCTL_PC6_WT1CCP0: u32 = 0x07000000; // WT1CCP0 on PC6
const GPIO_PCTL_PC5_M: u32 = 0x00F00000; // PC5 mask
const GPIO_PCTL_PC5_U4TX: u32 = 0x00100000; // U4TX on PC5
const GPIO_PCTL_PC5_U1TX: u32 = 0x00200000; // U1TX on PC5
const GPIO_PCTL_PC5_WT0CCP1: u32 = 0x00700000; // WT0CCP1 on PC5
const GPIO_PCTL_PC5_U1CTS: u32 = 0x00800000; // U1CTS on PC5
const GPIO_PCTL_PC4_M: u32 = 0x000F0000; // PC4 mask
const GPIO_PCTL_PC4_U4RX: u32 = 0x00010000; // U4RX on PC4
const GPIO_PCTL_PC4_U1RX: u32 = 0x00020000; // U1RX on PC4
const GPIO_PCTL_PC4_WT0CCP0: u32 = 0x00070000; // WT0CCP0 on PC4
const GPIO_PCTL_PC4_U1RTS: u32 = 0x00080000; // U1RTS on PC4
const GPIO_PCTL_PC3_M: u32 = 0x0000F000; // PC3 mask
const GPIO_PCTL_PC3_TDO: u32 = 0x00001000; // TDO on PC3
const GPIO_PCTL_PC3_T5CCP1: u32 = 0x00007000; // T5CCP1 on PC3
const GPIO_PCTL_PC2_M: u32 = 0x00000F00; // PC2 mask
const GPIO_PCTL_PC2_TDI: u32 = 0x00000100; // TDI on PC2
const GPIO_PCTL_PC2_T5CCP0: u32 = 0x00000700; // T5CCP0 on PC2
const GPIO_PCTL_PC1_M: u32 = 0x000000F0; // PC1 mask
const GPIO_PCTL_PC1_TMS: u32 = 0x00000010; // TMS on PC1
const GPIO_PCTL_PC1_T4CCP1: u32 = 0x00000070; // T4CCP1 on PC1
const GPIO_PCTL_PC0_M: u32 = 0x0000000F; // PC0 mask
const GPIO_PCTL_PC0_TCK: u32 = 0x00000001; // TCK on PC0
const GPIO_PCTL_PC0_T4CCP0: u32 = 0x00000007; // T4CCP0 on PC0

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port D.
//
// *****************************************************************************
const GPIO_PCTL_PD7_M: u32 = 0xF0000000; // PD7 mask
const GPIO_PCTL_PD7_U2TX: u32 = 0x10000000; // U2TX on PD7
const GPIO_PCTL_PD7_WT5CCP1: u32 = 0x70000000; // WT5CCP1 on PD7
const GPIO_PCTL_PD7_NMI: u32 = 0x80000000; // NMI on PD7
const GPIO_PCTL_PD6_M: u32 = 0x0F000000; // PD6 mask
const GPIO_PCTL_PD6_U2RX: u32 = 0x01000000; // U2RX on PD6
const GPIO_PCTL_PD6_WT5CCP0: u32 = 0x07000000; // WT5CCP0 on PD6
const GPIO_PCTL_PD5_M: u32 = 0x00F00000; // PD5 mask
const GPIO_PCTL_PD5_U6TX: u32 = 0x00100000; // U6TX on PD5
const GPIO_PCTL_PD5_WT4CCP1: u32 = 0x00700000; // WT4CCP1 on PD5
const GPIO_PCTL_PD4_M: u32 = 0x000F0000; // PD4 mask
const GPIO_PCTL_PD4_U6RX: u32 = 0x00010000; // U6RX on PD4
const GPIO_PCTL_PD4_WT4CCP0: u32 = 0x00070000; // WT4CCP0 on PD4
const GPIO_PCTL_PD3_M: u32 = 0x0000F000; // PD3 mask
const GPIO_PCTL_PD3_SSI3TX: u32 = 0x00001000; // SSI3TX on PD3
const GPIO_PCTL_PD3_SSI1TX: u32 = 0x00002000; // SSI1TX on PD3
const GPIO_PCTL_PD3_WT3CCP1: u32 = 0x00007000; // WT3CCP1 on PD3
const GPIO_PCTL_PD2_M: u32 = 0x00000F00; // PD2 mask
const GPIO_PCTL_PD2_SSI3RX: u32 = 0x00000100; // SSI3RX on PD2
const GPIO_PCTL_PD2_SSI1RX: u32 = 0x00000200; // SSI1RX on PD2
const GPIO_PCTL_PD2_WT3CCP0: u32 = 0x00000700; // WT3CCP0 on PD2
const GPIO_PCTL_PD1_M: u32 = 0x000000F0; // PD1 mask
const GPIO_PCTL_PD1_SSI3FSS: u32 = 0x00000010; // SSI3FSS on PD1
const GPIO_PCTL_PD1_SSI1FSS: u32 = 0x00000020; // SSI1FSS on PD1
const GPIO_PCTL_PD1_I2C3SDA: u32 = 0x00000030; // I2C3SDA on PD1
const GPIO_PCTL_PD1_WT2CCP1: u32 = 0x00000070; // WT2CCP1 on PD1
const GPIO_PCTL_PD0_M: u32 = 0x0000000F; // PD0 mask
const GPIO_PCTL_PD0_SSI3CLK: u32 = 0x00000001; // SSI3CLK on PD0
const GPIO_PCTL_PD0_SSI1CLK: u32 = 0x00000002; // SSI1CLK on PD0
const GPIO_PCTL_PD0_I2C3SCL: u32 = 0x00000003; // I2C3SCL on PD0
const GPIO_PCTL_PD0_WT2CCP0: u32 = 0x00000007; // WT2CCP0 on PD0

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port E.
//
// *****************************************************************************
const GPIO_PCTL_PE5_M: u32 = 0x00F00000; // PE5 mask
const GPIO_PCTL_PE5_U5TX: u32 = 0x00100000; // U5TX on PE5
const GPIO_PCTL_PE5_I2C2SDA: u32 = 0x00300000; // I2C2SDA on PE5
const GPIO_PCTL_PE5_CAN0TX: u32 = 0x00800000; // CAN0TX on PE5
const GPIO_PCTL_PE4_M: u32 = 0x000F0000; // PE4 mask
const GPIO_PCTL_PE4_U5RX: u32 = 0x00010000; // U5RX on PE4
const GPIO_PCTL_PE4_I2C2SCL: u32 = 0x00030000; // I2C2SCL on PE4
const GPIO_PCTL_PE4_CAN0RX: u32 = 0x00080000; // CAN0RX on PE4
const GPIO_PCTL_PE3_M: u32 = 0x0000F000; // PE3 mask
const GPIO_PCTL_PE2_M: u32 = 0x00000F00; // PE2 mask
const GPIO_PCTL_PE1_M: u32 = 0x000000F0; // PE1 mask
const GPIO_PCTL_PE1_U7TX: u32 = 0x00000010; // U7TX on PE1
const GPIO_PCTL_PE0_M: u32 = 0x0000000F; // PE0 mask
const GPIO_PCTL_PE0_U7RX: u32 = 0x00000001; // U7RX on PE0

// *****************************************************************************
//
// The following are defines for the bit fields in the GPIO_PCTL register for
// port F.
//
// *****************************************************************************
const GPIO_PCTL_PF4_M: u32 = 0x000F0000; // PF4 mask
const GPIO_PCTL_PF4_T2CCP0: u32 = 0x00070000; // T2CCP0 on PF4
const GPIO_PCTL_PF3_M: u32 = 0x0000F000; // PF3 mask
const GPIO_PCTL_PF3_SSI1FSS: u32 = 0x00002000; // SSI1FSS on PF3
const GPIO_PCTL_PF3_CAN0TX: u32 = 0x00003000; // CAN0TX on PF3
const GPIO_PCTL_PF3_T1CCP1: u32 = 0x00007000; // T1CCP1 on PF3
const GPIO_PCTL_PF3_TRCLK: u32 = 0x0000E000; // TRCLK on PF3
const GPIO_PCTL_PF2_M: u32 = 0x00000F00; // PF2 mask
const GPIO_PCTL_PF2_SSI1CLK: u32 = 0x00000200; // SSI1CLK on PF2
const GPIO_PCTL_PF2_T1CCP0: u32 = 0x00000700; // T1CCP0 on PF2
const GPIO_PCTL_PF2_TRD0: u32 = 0x00000E00; // TRD0 on PF2
const GPIO_PCTL_PF1_M: u32 = 0x000000F0; // PF1 mask
const GPIO_PCTL_PF1_U1CTS: u32 = 0x00000010; // U1CTS on PF1
const GPIO_PCTL_PF1_SSI1TX: u32 = 0x00000020; // SSI1TX on PF1
const GPIO_PCTL_PF1_T0CCP1: u32 = 0x00000070; // T0CCP1 on PF1
const GPIO_PCTL_PF1_C1O: u32 = 0x00000090; // C1O on PF1
const GPIO_PCTL_PF1_TRD1: u32 = 0x000000E0; // TRD1 on PF1
const GPIO_PCTL_PF0_M: u32 = 0x0000000F; // PF0 mask
const GPIO_PCTL_PF0_U1RTS: u32 = 0x00000001; // U1RTS on PF0
const GPIO_PCTL_PF0_SSI1RX: u32 = 0x00000002; // SSI1RX on PF0
const GPIO_PCTL_PF0_CAN0RX: u32 = 0x00000003; // CAN0RX on PF0
const GPIO_PCTL_PF0_T0CCP0: u32 = 0x00000007; // T0CCP0 on PF0
const GPIO_PCTL_PF0_NMI: u32 = 0x00000008; // NMI on PF0
const GPIO_PCTL_PF0_C0O: u32 = 0x00000009; // C0O on PF0
const GPIO_PCTL_PF0_TRD2: u32 = 0x0000000E; // TRD2 on PF0

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_CR0 register.
//
// *****************************************************************************
const SSI_CR0_SCR_M: u32 = 0x0000FF00; // SSI Serial Clock Rate
const SSI_CR0_SPH: u32 = 0x00000080; // SSI Serial Clock Phase
const SSI_CR0_SPO: u32 = 0x00000040; // SSI Serial Clock Polarity
const SSI_CR0_FRF_M: u32 = 0x00000030; // SSI Frame Format Select
const SSI_CR0_FRF_MOTO: u32 = 0x00000000; // Freescale SPI Frame Format
const SSI_CR0_FRF_TI: u32 = 0x00000010; // Texas Instruments Synchronous Serial Frame Format
const SSI_CR0_FRF_NMW: u32 = 0x00000020; // MICROWIRE Frame Format
const SSI_CR0_DSS_M: u32 = 0x0000000F; // SSI Data Size Select
const SSI_CR0_DSS_4: u32 = 0x00000003; // 4-bit data
const SSI_CR0_DSS_5: u32 = 0x00000004; // 5-bit data
const SSI_CR0_DSS_6: u32 = 0x00000005; // 6-bit data
const SSI_CR0_DSS_7: u32 = 0x00000006; // 7-bit data
const SSI_CR0_DSS_8: u32 = 0x00000007; // 8-bit data
const SSI_CR0_DSS_9: u32 = 0x00000008; // 9-bit data
const SSI_CR0_DSS_10: u32 = 0x00000009; // 10-bit data
const SSI_CR0_DSS_11: u32 = 0x0000000A; // 11-bit data
const SSI_CR0_DSS_12: u32 = 0x0000000B; // 12-bit data
const SSI_CR0_DSS_13: u32 = 0x0000000C; // 13-bit data
const SSI_CR0_DSS_14: u32 = 0x0000000D; // 14-bit data
const SSI_CR0_DSS_15: u32 = 0x0000000E; // 15-bit data
const SSI_CR0_DSS_16: u32 = 0x0000000F; // 16-bit data
const SSI_CR0_SCR_S: u32 = 8;

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_CR1 register.
//
// *****************************************************************************
const SSI_CR1_EOT: u32 = 0x00000010; // End of Transmission
const SSI_CR1_SOD: u32 = 0x00000008; // SSI Slave Mode Output Disable
const SSI_CR1_MS: u32 = 0x00000004; // SSI Master/Slave Select
const SSI_CR1_SSE: u32 = 0x00000002; // SSI Synchronous Serial Port Enable
const SSI_CR1_LBM: u32 = 0x00000001; // SSI Loopback Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_DR register.
//
// *****************************************************************************
const SSI_DR_DATA_M: u32 = 0x0000FFFF; // SSI Receive/Transmit Data
const SSI_DR_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_SR register.
//
// *****************************************************************************
const SSI_SR_BSY: u32 = 0x00000010; // SSI Busy Bit
const SSI_SR_RFF: u32 = 0x00000008; // SSI Receive FIFO Full
const SSI_SR_RNE: u32 = 0x00000004; // SSI Receive FIFO Not Empty
const SSI_SR_TNF: u32 = 0x00000002; // SSI Transmit FIFO Not Full
const SSI_SR_TFE: u32 = 0x00000001; // SSI Transmit FIFO Empty

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_CPSR register.
//
// *****************************************************************************
const SSI_CPSR_CPSDVSR_M: u32 = 0x000000FF; // SSI Clock Prescale Divisor
const SSI_CPSR_CPSDVSR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_IM register.
//
// *****************************************************************************
const SSI_IM_TXIM: u32 = 0x00000008; // SSI Transmit FIFO Interrupt Mask
const SSI_IM_RXIM: u32 = 0x00000004; // SSI Receive FIFO Interrupt Mask
const SSI_IM_RTIM: u32 = 0x00000002; // SSI Receive Time-Out Interrupt Mask
const SSI_IM_RORIM: u32 = 0x00000001; // SSI Receive Overrun Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_RIS register.
//
// *****************************************************************************
const SSI_RIS_TXRIS: u32 = 0x00000008; // SSI Transmit FIFO Raw Interrupt Status
const SSI_RIS_RXRIS: u32 = 0x00000004; // SSI Receive FIFO Raw Interrupt Status
const SSI_RIS_RTRIS: u32 = 0x00000002; // SSI Receive Time-Out Raw Interrupt Status
const SSI_RIS_RORRIS: u32 = 0x00000001; // SSI Receive Overrun Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_MIS register.
//
// *****************************************************************************
const SSI_MIS_TXMIS: u32 = 0x00000008; // SSI Transmit FIFO Masked Interrupt Status
const SSI_MIS_RXMIS: u32 = 0x00000004; // SSI Receive FIFO Masked Interrupt Status
const SSI_MIS_RTMIS: u32 = 0x00000002; // SSI Receive Time-Out Masked Interrupt Status
const SSI_MIS_RORMIS: u32 = 0x00000001; // SSI Receive Overrun Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_ICR register.
//
// *****************************************************************************
const SSI_ICR_RTIC: u32 = 0x00000002; // SSI Receive Time-Out Interrupt Clear
const SSI_ICR_RORIC: u32 = 0x00000001; // SSI Receive Overrun Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_DMACTL register.
//
// *****************************************************************************
const SSI_DMACTL_TXDMAE: u32 = 0x00000002; // Transmit DMA Enable
const SSI_DMACTL_RXDMAE: u32 = 0x00000001; // Receive DMA Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the SSI_O_CC register.
//
// *****************************************************************************
const SSI_CC_CS_M: u32 = 0x0000000F; // SSI Baud Clock Source
const SSI_CC_CS_SYSPLL: u32 = 0x00000000; // Either the system clock (if the PLL bypass is in effect) or the PLL output (default)
const SSI_CC_CS_PIOSC: u32 = 0x00000005; // PIOSC

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_DR register.
//
// *****************************************************************************
const UART_DR_OE: u32 = 0x00000800; // UART Overrun Error
const UART_DR_BE: u32 = 0x00000400; // UART Break Error
const UART_DR_PE: u32 = 0x00000200; // UART Parity Error
const UART_DR_FE: u32 = 0x00000100; // UART Framing Error
const UART_DR_DATA_M: u32 = 0x000000FF; // Data Transmitted or Received
const UART_DR_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_RSR register.
//
// *****************************************************************************
const UART_RSR_OE: u32 = 0x00000008; // UART Overrun Error
const UART_RSR_BE: u32 = 0x00000004; // UART Break Error
const UART_RSR_PE: u32 = 0x00000002; // UART Parity Error
const UART_RSR_FE: u32 = 0x00000001; // UART Framing Error

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_ECR register.
//
// *****************************************************************************
const UART_ECR_DATA_M: u32 = 0x000000FF; // Error Clear
const UART_ECR_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_FR register.
//
// *****************************************************************************
const UART_FR_TXFE: u32 = 0x00000080; // UART Transmit FIFO Empty
const UART_FR_RXFF: u32 = 0x00000040; // UART Receive FIFO Full
const UART_FR_TXFF: u32 = 0x00000020; // UART Transmit FIFO Full
const UART_FR_RXFE: u32 = 0x00000010; // UART Receive FIFO Empty
const UART_FR_BUSY: u32 = 0x00000008; // UART Busy
const UART_FR_CTS: u32 = 0x00000001; // Clear To Send

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_ILPR register.
//
// *****************************************************************************
const UART_ILPR_ILPDVSR_M: u32 = 0x000000FF; // IrDA Low-Power Divisor
const UART_ILPR_ILPDVSR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_IBRD register.
//
// *****************************************************************************
const UART_IBRD_DIVINT_M: u32 = 0x0000FFFF; // Integer Baud-Rate Divisor
const UART_IBRD_DIVINT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_FBRD register.
//
// *****************************************************************************
const UART_FBRD_DIVFRAC_M: u32 = 0x0000003F; // Fractional Baud-Rate Divisor
const UART_FBRD_DIVFRAC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_LCRH register.
//
// *****************************************************************************
const UART_LCRH_SPS: u32 = 0x00000080; // UART Stick Parity Select
const UART_LCRH_WLEN_M: u32 = 0x00000060; // UART Word Length
const UART_LCRH_WLEN_5: u32 = 0x00000000; // 5 bits (default)
const UART_LCRH_WLEN_6: u32 = 0x00000020; // 6 bits
const UART_LCRH_WLEN_7: u32 = 0x00000040; // 7 bits
const UART_LCRH_WLEN_8: u32 = 0x00000060; // 8 bits
const UART_LCRH_FEN: u32 = 0x00000010; // UART Enable FIFOs
const UART_LCRH_STP2: u32 = 0x00000008; // UART Two Stop Bits Select
const UART_LCRH_EPS: u32 = 0x00000004; // UART Even Parity Select
const UART_LCRH_PEN: u32 = 0x00000002; // UART Parity Enable
const UART_LCRH_BRK: u32 = 0x00000001; // UART Send Break

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_CTL register.
//
// *****************************************************************************
const UART_CTL_RXE: u32 = 0x00000200; // UART Receive Enable
const UART_CTL_TXE: u32 = 0x00000100; // UART Transmit Enable
const UART_CTL_LBE: u32 = 0x00000080; // UART Loop Back Enable
const UART_CTL_LIN: u32 = 0x00000040; // LIN Mode Enable
const UART_CTL_HSE: u32 = 0x00000020; // High-Speed Enable
const UART_CTL_EOT: u32 = 0x00000010; // End of Transmission
const UART_CTL_SMART: u32 = 0x00000008; // ISO 7816 Smart Card Support
const UART_CTL_SIRLP: u32 = 0x00000004; // UART SIR Low-Power Mode
const UART_CTL_SIREN: u32 = 0x00000002; // UART SIR Enable
const UART_CTL_UARTEN: u32 = 0x00000001; // UART Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_IFLS register.
//
// *****************************************************************************
const UART_IFLS_RX_M: u32 = 0x00000038; // UART Receive Interrupt FIFO Level Select
const UART_IFLS_RX1_8: u32 = 0x00000000; // RX FIFO >= 1/8 full
const UART_IFLS_RX2_8: u32 = 0x00000008; // RX FIFO >= 1/4 full
const UART_IFLS_RX4_8: u32 = 0x00000010; // RX FIFO >= 1/2 full (default)
const UART_IFLS_RX6_8: u32 = 0x00000018; // RX FIFO >= 3/4 full
const UART_IFLS_RX7_8: u32 = 0x00000020; // RX FIFO >= 7/8 full
const UART_IFLS_TX_M: u32 = 0x00000007; // UART Transmit Interrupt FIFO Level Select
const UART_IFLS_TX1_8: u32 = 0x00000000; // TX FIFO <= 1/8 full
const UART_IFLS_TX2_8: u32 = 0x00000001; // TX FIFO <= 1/4 full
const UART_IFLS_TX4_8: u32 = 0x00000002; // TX FIFO <= 1/2 full (default)
const UART_IFLS_TX6_8: u32 = 0x00000003; // TX FIFO <= 3/4 full
const UART_IFLS_TX7_8: u32 = 0x00000004; // TX FIFO <= 7/8 full

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_IM register.
//
// *****************************************************************************
const UART_IM_LME5IM: u32 = 0x00008000; // LIN Mode Edge 5 Interrupt Mask
const UART_IM_LME1IM: u32 = 0x00004000; // LIN Mode Edge 1 Interrupt Mask
const UART_IM_LMSBIM: u32 = 0x00002000; // LIN Mode Sync Break Interrupt Mask
const UART_IM_9BITIM: u32 = 0x00001000; // 9-Bit Mode Interrupt Mask
const UART_IM_OEIM: u32 = 0x00000400; // UART Overrun Error Interrupt Mask
const UART_IM_BEIM: u32 = 0x00000200; // UART Break Error Interrupt Mask
const UART_IM_PEIM: u32 = 0x00000100; // UART Parity Error Interrupt Mask
const UART_IM_FEIM: u32 = 0x00000080; // UART Framing Error Interrupt Mask
const UART_IM_RTIM: u32 = 0x00000040; // UART Receive Time-Out Interrupt Mask
const UART_IM_TXIM: u32 = 0x00000020; // UART Transmit Interrupt Mask
const UART_IM_RXIM: u32 = 0x00000010; // UART Receive Interrupt Mask
const UART_IM_CTSMIM: u32 = 0x00000002; // UART Clear to Send Modem Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_RIS register.
//
// *****************************************************************************
const UART_RIS_LME5RIS: u32 = 0x00008000; // LIN Mode Edge 5 Raw Interrupt Status
const UART_RIS_LME1RIS: u32 = 0x00004000; // LIN Mode Edge 1 Raw Interrupt Status
const UART_RIS_LMSBRIS: u32 = 0x00002000; // LIN Mode Sync Break Raw Interrupt Status
const UART_RIS_9BITRIS: u32 = 0x00001000; // 9-Bit Mode Raw Interrupt Status
const UART_RIS_OERIS: u32 = 0x00000400; // UART Overrun Error Raw Interrupt Status
const UART_RIS_BERIS: u32 = 0x00000200; // UART Break Error Raw Interrupt Status
const UART_RIS_PERIS: u32 = 0x00000100; // UART Parity Error Raw Interrupt Status
const UART_RIS_FERIS: u32 = 0x00000080; // UART Framing Error Raw Interrupt Status
const UART_RIS_RTRIS: u32 = 0x00000040; // UART Receive Time-Out Raw Interrupt Status
const UART_RIS_TXRIS: u32 = 0x00000020; // UART Transmit Raw Interrupt Status
const UART_RIS_RXRIS: u32 = 0x00000010; // UART Receive Raw Interrupt Status
const UART_RIS_CTSRIS: u32 = 0x00000002; // UART Clear to Send Modem Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_MIS register.
//
// *****************************************************************************
const UART_MIS_LME5MIS: u32 = 0x00008000; // LIN Mode Edge 5 Masked Interrupt Status
const UART_MIS_LME1MIS: u32 = 0x00004000; // LIN Mode Edge 1 Masked Interrupt Status
const UART_MIS_LMSBMIS: u32 = 0x00002000; // LIN Mode Sync Break Masked Interrupt Status
const UART_MIS_9BITMIS: u32 = 0x00001000; // 9-Bit Mode Masked Interrupt Status
const UART_MIS_OEMIS: u32 = 0x00000400; // UART Overrun Error Masked Interrupt Status
const UART_MIS_BEMIS: u32 = 0x00000200; // UART Break Error Masked Interrupt Status
const UART_MIS_PEMIS: u32 = 0x00000100; // UART Parity Error Masked Interrupt Status
const UART_MIS_FEMIS: u32 = 0x00000080; // UART Framing Error Masked Interrupt Status
const UART_MIS_RTMIS: u32 = 0x00000040; // UART Receive Time-Out Masked Interrupt Status
const UART_MIS_TXMIS: u32 = 0x00000020; // UART Transmit Masked Interrupt Status
const UART_MIS_RXMIS: u32 = 0x00000010; // UART Receive Masked Interrupt Status
const UART_MIS_CTSMIS: u32 = 0x00000002; // UART Clear to Send Modem Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_ICR register.
//
// *****************************************************************************
const UART_ICR_LME5IC: u32 = 0x00008000; // LIN Mode Edge 5 Interrupt Clear
const UART_ICR_LME1IC: u32 = 0x00004000; // LIN Mode Edge 1 Interrupt Clear
const UART_ICR_LMSBIC: u32 = 0x00002000; // LIN Mode Sync Break Interrupt Clear
const UART_ICR_9BITIC: u32 = 0x00001000; // 9-Bit Mode Interrupt Clear
const UART_ICR_OEIC: u32 = 0x00000400; // Overrun Error Interrupt Clear
const UART_ICR_BEIC: u32 = 0x00000200; // Break Error Interrupt Clear
const UART_ICR_PEIC: u32 = 0x00000100; // Parity Error Interrupt Clear
const UART_ICR_FEIC: u32 = 0x00000080; // Framing Error Interrupt Clear
const UART_ICR_RTIC: u32 = 0x00000040; // Receive Time-Out Interrupt Clear
const UART_ICR_TXIC: u32 = 0x00000020; // Transmit Interrupt Clear
const UART_ICR_RXIC: u32 = 0x00000010; // Receive Interrupt Clear
const UART_ICR_CTSMIC: u32 = 0x00000002; // UART Clear to Send Modem Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_DMACTL register.
//
// *****************************************************************************
const UART_DMACTL_DMAERR: u32 = 0x00000004; // DMA on Error
const UART_DMACTL_TXDMAE: u32 = 0x00000002; // Transmit DMA Enable
const UART_DMACTL_RXDMAE: u32 = 0x00000001; // Receive DMA Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_LCTL register.
//
// *****************************************************************************
const UART_LCTL_BLEN_M: u32 = 0x00000030; // Sync Break Length
const UART_LCTL_BLEN_13T: u32 = 0x00000000; // Sync break length is 13T bits (default)
const UART_LCTL_BLEN_14T: u32 = 0x00000010; // Sync break length is 14T bits
const UART_LCTL_BLEN_15T: u32 = 0x00000020; // Sync break length is 15T bits
const UART_LCTL_BLEN_16T: u32 = 0x00000030; // Sync break length is 16T bits
const UART_LCTL_MASTER: u32 = 0x00000001; // LIN Master Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_LSS register.
//
// *****************************************************************************
const UART_LSS_TSS_M: u32 = 0x0000FFFF; // Timer Snap Shot
const UART_LSS_TSS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_LTIM register.
//
// *****************************************************************************
const UART_LTIM_TIMER_M: u32 = 0x0000FFFF; // Timer Value
const UART_LTIM_TIMER_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_9BITADDR
// register.
//
// *****************************************************************************
const UART_9BITADDR_9BITEN: u32 = 0x00008000; // Enable 9-Bit Mode
const UART_9BITADDR_ADDR_M: u32 = 0x000000FF; // Self Address for 9-Bit Mode
const UART_9BITADDR_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_9BITAMASK
// register.
//
// *****************************************************************************
const UART_9BITAMASK_MASK_M: u32 = 0x000000FF; // Self Address Mask for 9-Bit Mode
const UART_9BITAMASK_MASK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_PP register.
//
// *****************************************************************************
const UART_PP_NB: u32 = 0x00000002; // 9-Bit Support
const UART_PP_SC: u32 = 0x00000001; // Smart Card Support

// *****************************************************************************
//
// The following are defines for the bit fields in the UART_O_CC register.
//
// *****************************************************************************
const UART_CC_CS_M: u32 = 0x0000000F; // UART Baud Clock Source
const UART_CC_CS_SYSCLK: u32 = 0x00000000; // The system clock (default)
const UART_CC_CS_PIOSC: u32 = 0x00000005; // PIOSC

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MSA register.
//
// *****************************************************************************
const I2C_MSA_SA_M: u32 = 0x000000FE; // I2C Slave Address
const I2C_MSA_RS: u32 = 0x00000001; // Receive not send
const I2C_MSA_SA_S: u32 = 1;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SOAR register.
//
// *****************************************************************************
const I2C_SOAR_OAR_M: u32 = 0x0000007F; // I2C Slave Own Address
const I2C_SOAR_OAR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SCSR register.
//
// *****************************************************************************
const I2C_SCSR_OAR2SEL: u32 = 0x00000008; // OAR2 Address Matched
const I2C_SCSR_FBR: u32 = 0x00000004; // First Byte Received
const I2C_SCSR_TREQ: u32 = 0x00000002; // Transmit Request
const I2C_SCSR_DA: u32 = 0x00000001; // Device Active
const I2C_SCSR_RREQ: u32 = 0x00000001; // Receive Request

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MCS register.
//
// *****************************************************************************
const I2C_MCS_CLKTO: u32 = 0x00000080; // Clock Timeout Error
const I2C_MCS_BUSBSY: u32 = 0x00000040; // Bus Busy
const I2C_MCS_IDLE: u32 = 0x00000020; // I2C Idle
const I2C_MCS_ARBLST: u32 = 0x00000010; // Arbitration Lost
const I2C_MCS_HS: u32 = 0x00000010; // High-Speed Enable
const I2C_MCS_ACK: u32 = 0x00000008; // Data Acknowledge Enable
const I2C_MCS_DATACK: u32 = 0x00000008; // Acknowledge Data
const I2C_MCS_ADRACK: u32 = 0x00000004; // Acknowledge Address
const I2C_MCS_STOP: u32 = 0x00000004; // Generate STOP
const I2C_MCS_ERROR: u32 = 0x00000002; // Error
const I2C_MCS_START: u32 = 0x00000002; // Generate START
const I2C_MCS_RUN: u32 = 0x00000001; // I2C Master Enable
const I2C_MCS_BUSY: u32 = 0x00000001; // I2C Busy

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SDR register.
//
// *****************************************************************************
const I2C_SDR_DATA_M: u32 = 0x000000FF; // Data for Transfer
const I2C_SDR_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MDR register.
//
// *****************************************************************************
const I2C_MDR_DATA_M: u32 = 0x000000FF; // Data Transferred
const I2C_MDR_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MTPR register.
//
// *****************************************************************************
const I2C_MTPR_HS: u32 = 0x00000080; // High-Speed Enable
const I2C_MTPR_TPR_M: u32 = 0x0000007F; // SCL Clock Period
const I2C_MTPR_TPR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SIMR register.
//
// *****************************************************************************
const I2C_SIMR_STOPIM: u32 = 0x00000004; // Stop Condition Interrupt Mask
const I2C_SIMR_STARTIM: u32 = 0x00000002; // Start Condition Interrupt Mask
const I2C_SIMR_DATAIM: u32 = 0x00000001; // Data Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SRIS register.
//
// *****************************************************************************
const I2C_SRIS_STOPRIS: u32 = 0x00000004; // Stop Condition Raw Interrupt Status
const I2C_SRIS_STARTRIS: u32 = 0x00000002; // Start Condition Raw Interrupt Status
const I2C_SRIS_DATARIS: u32 = 0x00000001; // Data Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MIMR register.
//
// *****************************************************************************
const I2C_MIMR_CLKIM: u32 = 0x00000002; // Clock Timeout Interrupt Mask
const I2C_MIMR_IM: u32 = 0x00000001; // Master Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MRIS register.
//
// *****************************************************************************
const I2C_MRIS_CLKRIS: u32 = 0x00000002; // Clock Timeout Raw Interrupt Status
const I2C_MRIS_RIS: u32 = 0x00000001; // Master Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SMIS register.
//
// *****************************************************************************
const I2C_SMIS_STOPMIS: u32 = 0x00000004; // Stop Condition Masked Interrupt Status
const I2C_SMIS_STARTMIS: u32 = 0x00000002; // Start Condition Masked Interrupt Status
const I2C_SMIS_DATAMIS: u32 = 0x00000001; // Data Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SICR register.
//
// *****************************************************************************
const I2C_SICR_STOPIC: u32 = 0x00000004; // Stop Condition Interrupt Clear
const I2C_SICR_STARTIC: u32 = 0x00000002; // Start Condition Interrupt Clear
const I2C_SICR_DATAIC: u32 = 0x00000001; // Data Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MMIS register.
//
// *****************************************************************************
const I2C_MMIS_CLKMIS: u32 = 0x00000002; // Clock Timeout Masked Interrupt Status
const I2C_MMIS_MIS: u32 = 0x00000001; // Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MICR register.
//
// *****************************************************************************
const I2C_MICR_CLKIC: u32 = 0x00000002; // Clock Timeout Interrupt Clear
const I2C_MICR_IC: u32 = 0x00000001; // Master Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SOAR2 register.
//
// *****************************************************************************
const I2C_SOAR2_OAR2EN: u32 = 0x00000080; // I2C Slave Own Address 2 Enable
const I2C_SOAR2_OAR2_M: u32 = 0x0000007F; // I2C Slave Own Address 2
const I2C_SOAR2_OAR2_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MCR register.
//
// *****************************************************************************
const I2C_MCR_SFE: u32 = 0x00000020; // I2C Slave Function Enable
const I2C_MCR_MFE: u32 = 0x00000010; // I2C Master Function Enable
const I2C_MCR_LPBK: u32 = 0x00000001; // I2C Loopback

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_SACKCTL register.
//
// *****************************************************************************
const I2C_SACKCTL_ACKOVAL: u32 = 0x00000002; // I2C Slave ACK Override Value
const I2C_SACKCTL_ACKOEN: u32 = 0x00000001; // I2C Slave ACK Override Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MCLKOCNT register.
//
// *****************************************************************************
const I2C_MCLKOCNT_CNTL_M: u32 = 0x000000FF; // I2C Master Count
const I2C_MCLKOCNT_CNTL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_MBMON register.
//
// *****************************************************************************
const I2C_MBMON_SDA: u32 = 0x00000002; // I2C SDA Status
const I2C_MBMON_SCL: u32 = 0x00000001; // I2C SCL Status

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_PP register.
//
// *****************************************************************************
const I2C_PP_HS: u32 = 0x00000001; // High-Speed Capable

// *****************************************************************************
//
// The following are defines for the bit fields in the I2C_O_PC register.
//
// *****************************************************************************
const I2C_PC_HS: u32 = 0x00000001; // High-Speed Capable

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_CFG register.
//
// *****************************************************************************
const TIMER_CFG_M: u32 = 0x00000007; // GPTM Configuration
const TIMER_CFG_32_BIT_TIMER: u32 = 0x00000000; // 32-bit timer configuration
const TIMER_CFG_32_BIT_RTC: u32 = 0x00000001; // 32-bit real-time clock (RTC) counter configuration
const TIMER_CFG_16_BIT: u32 = 0x00000004; // 16-bit timer configuration. The function is controlled by bits 1:0 of GPTMTAMR and GPTMTBMR

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAMR register.
//
// *****************************************************************************
const TIMER_TAMR_TAPLO: u32 = 0x00000800; // GPTM Timer A PWM Legacy Operation
const TIMER_TAMR_TAMRSU: u32 = 0x00000400; // GPTM Timer A Match Register Update
const TIMER_TAMR_TAPWMIE: u32 = 0x00000200; // GPTM Timer A PWM Interrupt Enable
const TIMER_TAMR_TAILD: u32 = 0x00000100; // GPTM Timer A Interval Load Write
const TIMER_TAMR_TASNAPS: u32 = 0x00000080; // GPTM Timer A Snap-Shot Mode
const TIMER_TAMR_TAWOT: u32 = 0x00000040; // GPTM Timer A Wait-on-Trigger
const TIMER_TAMR_TAMIE: u32 = 0x00000020; // GPTM Timer A Match Interrupt Enable
const TIMER_TAMR_TACDIR: u32 = 0x00000010; // GPTM Timer A Count Direction
const TIMER_TAMR_TAAMS: u32 = 0x00000008; // GPTM Timer A Alternate Mode Select
const TIMER_TAMR_TACMR: u32 = 0x00000004; // GPTM Timer A Capture Mode
const TIMER_TAMR_TAMR_M: u32 = 0x00000003; // GPTM Timer A Mode
const TIMER_TAMR_TAMR_1_SHOT: u32 = 0x00000001; // One-Shot Timer mode
const TIMER_TAMR_TAMR_PERIOD: u32 = 0x00000002; // Periodic Timer mode
const TIMER_TAMR_TAMR_CAP: u32 = 0x00000003; // Capture mode

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBMR register.
//
// *****************************************************************************
const TIMER_TBMR_TBPLO: u32 = 0x00000800; // GPTM Timer B PWM Legacy Operation
const TIMER_TBMR_TBMRSU: u32 = 0x00000400; // GPTM Timer B Match Register Update
const TIMER_TBMR_TBPWMIE: u32 = 0x00000200; // GPTM Timer B PWM Interrupt Enable
const TIMER_TBMR_TBILD: u32 = 0x00000100; // GPTM Timer B Interval Load Write
const TIMER_TBMR_TBSNAPS: u32 = 0x00000080; // GPTM Timer B Snap-Shot Mode
const TIMER_TBMR_TBWOT: u32 = 0x00000040; // GPTM Timer B Wait-on-Trigger
const TIMER_TBMR_TBMIE: u32 = 0x00000020; // GPTM Timer B Match Interrupt Enable
const TIMER_TBMR_TBCDIR: u32 = 0x00000010; // GPTM Timer B Count Direction
const TIMER_TBMR_TBAMS: u32 = 0x00000008; // GPTM Timer B Alternate Mode Select
const TIMER_TBMR_TBCMR: u32 = 0x00000004; // GPTM Timer B Capture Mode
const TIMER_TBMR_TBMR_M: u32 = 0x00000003; // GPTM Timer B Mode
const TIMER_TBMR_TBMR_1_SHOT: u32 = 0x00000001; // One-Shot Timer mode
const TIMER_TBMR_TBMR_PERIOD: u32 = 0x00000002; // Periodic Timer mode
const TIMER_TBMR_TBMR_CAP: u32 = 0x00000003; // Capture mode

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_CTL register.
//
// *****************************************************************************
const TIMER_CTL_TBPWML: u32 = 0x00004000; // GPTM Timer B PWM Output Level
const TIMER_CTL_TBOTE: u32 = 0x00002000; // GPTM Timer B Output Trigger Enable
const TIMER_CTL_TBEVENT_M: u32 = 0x00000C00; // GPTM Timer B Event Mode
const TIMER_CTL_TBEVENT_POS: u32 = 0x00000000; // Positive edge
const TIMER_CTL_TBEVENT_NEG: u32 = 0x00000400; // Negative edge
const TIMER_CTL_TBEVENT_BOTH: u32 = 0x00000C00; // Both edges
const TIMER_CTL_TBSTALL: u32 = 0x00000200; // GPTM Timer B Stall Enable
const TIMER_CTL_TBEN: u32 = 0x00000100; // GPTM Timer B Enable
const TIMER_CTL_TAPWML: u32 = 0x00000040; // GPTM Timer A PWM Output Level
const TIMER_CTL_TAOTE: u32 = 0x00000020; // GPTM Timer A Output Trigger Enable
const TIMER_CTL_RTCEN: u32 = 0x00000010; // GPTM RTC Stall Enable
const TIMER_CTL_TAEVENT_M: u32 = 0x0000000C; // GPTM Timer A Event Mode
const TIMER_CTL_TAEVENT_POS: u32 = 0x00000000; // Positive edge
const TIMER_CTL_TAEVENT_NEG: u32 = 0x00000004; // Negative edge
const TIMER_CTL_TAEVENT_BOTH: u32 = 0x0000000C; // Both edges
const TIMER_CTL_TASTALL: u32 = 0x00000002; // GPTM Timer A Stall Enable
const TIMER_CTL_TAEN: u32 = 0x00000001; // GPTM Timer A Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_SYNC register.
//
// *****************************************************************************
const TIMER_SYNC_SYNCWT5_M: u32 = 0x00C00000; // Synchronize GPTM 32/64-Bit Timer 5
const TIMER_SYNC_SYNCWT5_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 5 is not affected
const TIMER_SYNC_SYNCWT5_TA: u32 = 0x00400000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCWT5_TB: u32 = 0x00800000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCWT5_TATB: u32 = 0x00C00000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCWT4_M: u32 = 0x00300000; // Synchronize GPTM 32/64-Bit Timer 4
const TIMER_SYNC_SYNCWT4_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 4 is not affected
const TIMER_SYNC_SYNCWT4_TA: u32 = 0x00100000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCWT4_TB: u32 = 0x00200000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCWT4_TATB: u32 = 0x00300000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCWT3_M: u32 = 0x000C0000; // Synchronize GPTM 32/64-Bit Timer 3
const TIMER_SYNC_SYNCWT3_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 3 is not affected
const TIMER_SYNC_SYNCWT3_TA: u32 = 0x00040000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCWT3_TB: u32 = 0x00080000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCWT3_TATB: u32 = 0x000C0000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCWT2_M: u32 = 0x00030000; // Synchronize GPTM 32/64-Bit Timer 2
const TIMER_SYNC_SYNCWT2_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 2 is not affected
const TIMER_SYNC_SYNCWT2_TA: u32 = 0x00010000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCWT2_TB: u32 = 0x00020000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCWT2_TATB: u32 = 0x00030000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCWT1_M: u32 = 0x0000C000; // Synchronize GPTM 32/64-Bit Timer 1
const TIMER_SYNC_SYNCWT1_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 1 is not affected
const TIMER_SYNC_SYNCWT1_TA: u32 = 0x00004000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCWT1_TB: u32 = 0x00008000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCWT1_TATB: u32 = 0x0000C000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCWT0_M: u32 = 0x00003000; // Synchronize GPTM 32/64-Bit Timer 0
const TIMER_SYNC_SYNCWT0_NONE: u32 = 0x00000000; // GPTM 32/64-Bit Timer 0 is not affected
const TIMER_SYNC_SYNCWT0_TA: u32 = 0x00001000; // A timeout event for Timer A of GPTM 32/64-Bit Timer 0 is triggered
const TIMER_SYNC_SYNCWT0_TB: u32 = 0x00002000; // A timeout event for Timer B of GPTM 32/64-Bit Timer 0 is triggered
const TIMER_SYNC_SYNCWT0_TATB: u32 = 0x00003000; // A timeout event for both Timer A and Timer B of GPTM 32/64-Bit Timer 0 is triggered
const TIMER_SYNC_SYNCT5_M: u32 = 0x00000C00; // Synchronize GPTM 16/32-Bit Timer 5
const TIMER_SYNC_SYNCT5_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 5 is not affected
const TIMER_SYNC_SYNCT5_TA: u32 = 0x00000400; // A timeout event for Timer A of GPTM 16/32-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCT5_TB: u32 = 0x00000800; // A timeout event for Timer B of GPTM 16/32-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCT5_TATB: u32 = 0x00000C00; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 5 is triggered
const TIMER_SYNC_SYNCT4_M: u32 = 0x00000300; // Synchronize GPTM 16/32-Bit Timer 4
const TIMER_SYNC_SYNCT4_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 4 is not affected
const TIMER_SYNC_SYNCT4_TA: u32 = 0x00000100; // A timeout event for Timer A of GPTM 16/32-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCT4_TB: u32 = 0x00000200; // A timeout event for Timer B of GPTM 16/32-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCT4_TATB: u32 = 0x00000300; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 4 is triggered
const TIMER_SYNC_SYNCT3_M: u32 = 0x000000C0; // Synchronize GPTM 16/32-Bit Timer 3
const TIMER_SYNC_SYNCT3_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 3 is not affected
const TIMER_SYNC_SYNCT3_TA: u32 = 0x00000040; // A timeout event for Timer A of GPTM 16/32-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCT3_TB: u32 = 0x00000080; // A timeout event for Timer B of GPTM 16/32-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCT3_TATB: u32 = 0x000000C0; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 3 is triggered
const TIMER_SYNC_SYNCT2_M: u32 = 0x00000030; // Synchronize GPTM 16/32-Bit Timer 2
const TIMER_SYNC_SYNCT2_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 2 is not affected
const TIMER_SYNC_SYNCT2_TA: u32 = 0x00000010; // A timeout event for Timer A of GPTM 16/32-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCT2_TB: u32 = 0x00000020; // A timeout event for Timer B of GPTM 16/32-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCT2_TATB: u32 = 0x00000030; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 2 is triggered
const TIMER_SYNC_SYNCT1_M: u32 = 0x0000000C; // Synchronize GPTM 16/32-Bit Timer 1
const TIMER_SYNC_SYNCT1_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 1 is not affected
const TIMER_SYNC_SYNCT1_TA: u32 = 0x00000004; // A timeout event for Timer A of GPTM 16/32-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCT1_TB: u32 = 0x00000008; // A timeout event for Timer B of GPTM 16/32-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCT1_TATB: u32 = 0x0000000C; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 1 is triggered
const TIMER_SYNC_SYNCT0_M: u32 = 0x00000003; // Synchronize GPTM 16/32-Bit Timer 0
const TIMER_SYNC_SYNCT0_NONE: u32 = 0x00000000; // GPTM 16/32-Bit Timer 0 is not affected
const TIMER_SYNC_SYNCT0_TA: u32 = 0x00000001; // A timeout event for Timer A of GPTM 16/32-Bit Timer 0 is triggered
const TIMER_SYNC_SYNCT0_TB: u32 = 0x00000002; // A timeout event for Timer B of GPTM 16/32-Bit Timer 0 is triggered
const TIMER_SYNC_SYNCT0_TATB: u32 = 0x00000003; // A timeout event for both Timer A and Timer B of GPTM 16/32-Bit Timer 0 is triggered

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_IMR register.
//
// *****************************************************************************
const TIMER_IMR_WUEIM: u32 = 0x00010000; // GPTM Write Update Error Interrupt Mask
const TIMER_IMR_TBMIM: u32 = 0x00000800; // GPTM Timer B Match Interrupt Mask
const TIMER_IMR_CBEIM: u32 = 0x00000400; // GPTM Timer B Capture Mode Event Interrupt Mask
const TIMER_IMR_CBMIM: u32 = 0x00000200; // GPTM Timer B Capture Mode Match Interrupt Mask
const TIMER_IMR_TBTOIM: u32 = 0x00000100; // GPTM Timer B Time-Out Interrupt Mask
const TIMER_IMR_TAMIM: u32 = 0x00000010; // GPTM Timer A Match Interrupt Mask
const TIMER_IMR_RTCIM: u32 = 0x00000008; // GPTM RTC Interrupt Mask
const TIMER_IMR_CAEIM: u32 = 0x00000004; // GPTM Timer A Capture Mode Event Interrupt Mask
const TIMER_IMR_CAMIM: u32 = 0x00000002; // GPTM Timer A Capture Mode Match Interrupt Mask
const TIMER_IMR_TATOIM: u32 = 0x00000001; // GPTM Timer A Time-Out Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_RIS register.
//
// *****************************************************************************
const TIMER_RIS_WUERIS: u32 = 0x00010000; // GPTM Write Update Error Raw Interrupt
const TIMER_RIS_TBMRIS: u32 = 0x00000800; // GPTM Timer B Match Raw Interrupt
const TIMER_RIS_CBERIS: u32 = 0x00000400; // GPTM Timer B Capture Mode Event Raw Interrupt
const TIMER_RIS_CBMRIS: u32 = 0x00000200; // GPTM Timer B Capture Mode Match Raw Interrupt
const TIMER_RIS_TBTORIS: u32 = 0x00000100; // GPTM Timer B Time-Out Raw Interrupt
const TIMER_RIS_TAMRIS: u32 = 0x00000010; // GPTM Timer A Match Raw Interrupt
const TIMER_RIS_RTCRIS: u32 = 0x00000008; // GPTM RTC Raw Interrupt
const TIMER_RIS_CAERIS: u32 = 0x00000004; // GPTM Timer A Capture Mode Event Raw Interrupt
const TIMER_RIS_CAMRIS: u32 = 0x00000002; // GPTM Timer A Capture Mode Match Raw Interrupt
const TIMER_RIS_TATORIS: u32 = 0x00000001; // GPTM Timer A Time-Out Raw Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_MIS register.
//
// *****************************************************************************
const TIMER_MIS_WUEMIS: u32 = 0x00010000; // GPTM Write Update Error Masked Interrupt
const TIMER_MIS_TBMMIS: u32 = 0x00000800; // GPTM Timer B Match Masked Interrupt
const TIMER_MIS_CBEMIS: u32 = 0x00000400; // GPTM Timer B Capture Mode Event Masked Interrupt
const TIMER_MIS_CBMMIS: u32 = 0x00000200; // GPTM Timer B Capture Mode Match Masked Interrupt
const TIMER_MIS_TBTOMIS: u32 = 0x00000100; // GPTM Timer B Time-Out Masked Interrupt
const TIMER_MIS_TAMMIS: u32 = 0x00000010; // GPTM Timer A Match Masked Interrupt
const TIMER_MIS_RTCMIS: u32 = 0x00000008; // GPTM RTC Masked Interrupt
const TIMER_MIS_CAEMIS: u32 = 0x00000004; // GPTM Timer A Capture Mode Event Masked Interrupt
const TIMER_MIS_CAMMIS: u32 = 0x00000002; // GPTM Timer A Capture Mode Match Masked Interrupt
const TIMER_MIS_TATOMIS: u32 = 0x00000001; // GPTM Timer A Time-Out Masked Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_ICR register.
//
// *****************************************************************************
const TIMER_ICR_WUECINT: u32 = 0x00010000; // 32/64-Bit GPTM Write Update Error Interrupt Clear
const TIMER_ICR_TBMCINT: u32 = 0x00000800; // GPTM Timer B Match Interrupt Clear
const TIMER_ICR_CBECINT: u32 = 0x00000400; // GPTM Timer B Capture Mode Event Interrupt Clear
const TIMER_ICR_CBMCINT: u32 = 0x00000200; // GPTM Timer B Capture Mode Match Interrupt Clear
const TIMER_ICR_TBTOCINT: u32 = 0x00000100; // GPTM Timer B Time-Out Interrupt Clear
const TIMER_ICR_TAMCINT: u32 = 0x00000010; // GPTM Timer A Match Interrupt Clear
const TIMER_ICR_RTCCINT: u32 = 0x00000008; // GPTM RTC Interrupt Clear
const TIMER_ICR_CAECINT: u32 = 0x00000004; // GPTM Timer A Capture Mode Event Interrupt Clear
const TIMER_ICR_CAMCINT: u32 = 0x00000002; // GPTM Timer A Capture Mode Match Interrupt Clear
const TIMER_ICR_TATOCINT: u32 = 0x00000001; // GPTM Timer A Time-Out Raw Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAILR register.
//
// *****************************************************************************
const TIMER_TAILR_M: u32 = 0xFFFFFFFF; // GPTM Timer A Interval Load Register
const TIMER_TAILR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBILR register.
//
// *****************************************************************************
const TIMER_TBILR_M: u32 = 0xFFFFFFFF; // GPTM Timer B Interval Load Register
const TIMER_TBILR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAMATCHR
// register.
//
// *****************************************************************************
const TIMER_TAMATCHR_TAMR_M: u32 = 0xFFFFFFFF; // GPTM Timer A Match Register
const TIMER_TAMATCHR_TAMR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBMATCHR
// register.
//
// *****************************************************************************
const TIMER_TBMATCHR_TBMR_M: u32 = 0xFFFFFFFF; // GPTM Timer B Match Register
const TIMER_TBMATCHR_TBMR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAPR register.
//
// *****************************************************************************
const TIMER_TAPR_TAPSRH_M: u32 = 0x0000FF00; // GPTM Timer A Prescale High Byte
const TIMER_TAPR_TAPSR_M: u32 = 0x000000FF; // GPTM Timer A Prescale
const TIMER_TAPR_TAPSRH_S: u32 = 8;
const TIMER_TAPR_TAPSR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBPR register.
//
// *****************************************************************************
const TIMER_TBPR_TBPSRH_M: u32 = 0x0000FF00; // GPTM Timer B Prescale High Byte
const TIMER_TBPR_TBPSR_M: u32 = 0x000000FF; // GPTM Timer B Prescale
const TIMER_TBPR_TBPSRH_S: u32 = 8;
const TIMER_TBPR_TBPSR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAPMR register.
//
// *****************************************************************************
const TIMER_TAPMR_TAPSMRH_M: u32 = 0x0000FF00; // GPTM Timer A Prescale Match High Byte
const TIMER_TAPMR_TAPSMR_M: u32 = 0x000000FF; // GPTM TimerA Prescale Match
const TIMER_TAPMR_TAPSMRH_S: u32 = 8;
const TIMER_TAPMR_TAPSMR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBPMR register.
//
// *****************************************************************************
const TIMER_TBPMR_TBPSMRH_M: u32 = 0x0000FF00; // GPTM Timer B Prescale Match High Byte
const TIMER_TBPMR_TBPSMR_M: u32 = 0x000000FF; // GPTM TimerB Prescale Match
const TIMER_TBPMR_TBPSMRH_S: u32 = 8;
const TIMER_TBPMR_TBPSMR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAR register.
//
// *****************************************************************************
const TIMER_TAR_M: u32 = 0xFFFFFFFF; // GPTM Timer A Register
const TIMER_TAR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBR register.
//
// *****************************************************************************
const TIMER_TBR_M: u32 = 0xFFFFFFFF; // GPTM Timer B Register
const TIMER_TBR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAV register.
//
// *****************************************************************************
const TIMER_TAV_M: u32 = 0xFFFFFFFF; // GPTM Timer A Value
const TIMER_TAV_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBV register.
//
// *****************************************************************************
const TIMER_TBV_M: u32 = 0xFFFFFFFF; // GPTM Timer B Value
const TIMER_TBV_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_RTCPD register.
//
// *****************************************************************************
const TIMER_RTCPD_RTCPD_M: u32 = 0x0000FFFF; // RTC Predivide Counter Value
const TIMER_RTCPD_RTCPD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAPS register.
//
// *****************************************************************************
const TIMER_TAPS_PSS_M: u32 = 0x0000FFFF; // GPTM Timer A Prescaler Snapshot
const TIMER_TAPS_PSS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBPS register.
//
// *****************************************************************************
const TIMER_TBPS_PSS_M: u32 = 0x0000FFFF; // GPTM Timer A Prescaler Value
const TIMER_TBPS_PSS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TAPV register.
//
// *****************************************************************************
const TIMER_TAPV_PSV_M: u32 = 0x0000FFFF; // GPTM Timer A Prescaler Value
const TIMER_TAPV_PSV_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_TBPV register.
//
// *****************************************************************************
const TIMER_TBPV_PSV_M: u32 = 0x0000FFFF; // GPTM Timer B Prescaler Value
const TIMER_TBPV_PSV_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the TIMER_O_PP register.
//
// *****************************************************************************
const TIMER_PP_SIZE_M: u32 = 0x0000000F; // Count Size
const TIMER_PP_SIZE_16: u32 = 0x00000000; // Timer A and Timer B counters are 16 bits each with an 8-bit prescale counter
const TIMER_PP_SIZE_32: u32 = 0x00000001; // Timer A and Timer B counters are 32 bits each with a 16-bit prescale counter

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_ACTSS register.
//
// *****************************************************************************
const ADC_ACTSS_ASEN3: u32 = 0x00000008; // ADC SS3 Enable
const ADC_ACTSS_ASEN2: u32 = 0x00000004; // ADC SS2 Enable
const ADC_ACTSS_ASEN1: u32 = 0x00000002; // ADC SS1 Enable
const ADC_ACTSS_ASEN0: u32 = 0x00000001; // ADC SS0 Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_RIS register.
//
// *****************************************************************************
const ADC_RIS_INRDC: u32 = 0x00010000; // Digital Comparator Raw Interrupt Status
const ADC_RIS_INR3: u32 = 0x00000008; // SS3 Raw Interrupt Status
const ADC_RIS_INR2: u32 = 0x00000004; // SS2 Raw Interrupt Status
const ADC_RIS_INR1: u32 = 0x00000002; // SS1 Raw Interrupt Status
const ADC_RIS_INR0: u32 = 0x00000001; // SS0 Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_IM register.
//
// *****************************************************************************
const ADC_IM_DCONSS3: u32 = 0x00080000; // Digital Comparator Interrupt on SS3
const ADC_IM_DCONSS2: u32 = 0x00040000; // Digital Comparator Interrupt on SS2
const ADC_IM_DCONSS1: u32 = 0x00020000; // Digital Comparator Interrupt on SS1
const ADC_IM_DCONSS0: u32 = 0x00010000; // Digital Comparator Interrupt on SS0
const ADC_IM_MASK3: u32 = 0x00000008; // SS3 Interrupt Mask
const ADC_IM_MASK2: u32 = 0x00000004; // SS2 Interrupt Mask
const ADC_IM_MASK1: u32 = 0x00000002; // SS1 Interrupt Mask
const ADC_IM_MASK0: u32 = 0x00000001; // SS0 Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_ISC register.
//
// *****************************************************************************
const ADC_ISC_DCINSS3: u32 = 0x00080000; // Digital Comparator Interrupt Status on SS3
const ADC_ISC_DCINSS2: u32 = 0x00040000; // Digital Comparator Interrupt Status on SS2
const ADC_ISC_DCINSS1: u32 = 0x00020000; // Digital Comparator Interrupt Status on SS1
const ADC_ISC_DCINSS0: u32 = 0x00010000; // Digital Comparator Interrupt Status on SS0
const ADC_ISC_IN3: u32 = 0x00000008; // SS3 Interrupt Status and Clear
const ADC_ISC_IN2: u32 = 0x00000004; // SS2 Interrupt Status and Clear
const ADC_ISC_IN1: u32 = 0x00000002; // SS1 Interrupt Status and Clear
const ADC_ISC_IN0: u32 = 0x00000001; // SS0 Interrupt Status and Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_OSTAT register.
//
// *****************************************************************************
const ADC_OSTAT_OV3: u32 = 0x00000008; // SS3 FIFO Overflow
const ADC_OSTAT_OV2: u32 = 0x00000004; // SS2 FIFO Overflow
const ADC_OSTAT_OV1: u32 = 0x00000002; // SS1 FIFO Overflow
const ADC_OSTAT_OV0: u32 = 0x00000001; // SS0 FIFO Overflow

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_EMUX register.
//
// *****************************************************************************
const ADC_EMUX_EM3_M: u32 = 0x0000F000; // SS3 Trigger Select
const ADC_EMUX_EM3_PROCESSOR: u32 = 0x00000000; // Processor (default)
const ADC_EMUX_EM3_COMP0: u32 = 0x00001000; // Analog Comparator 0
const ADC_EMUX_EM3_COMP1: u32 = 0x00002000; // Analog Comparator 1
const ADC_EMUX_EM3_EXTERNAL: u32 = 0x00004000; // External (GPIO PB4)
const ADC_EMUX_EM3_TIMER: u32 = 0x00005000; // Timer
const ADC_EMUX_EM3_ALWAYS: u32 = 0x0000F000; // Always (continuously sample)
const ADC_EMUX_EM2_M: u32 = 0x00000F00; // SS2 Trigger Select
const ADC_EMUX_EM2_PROCESSOR: u32 = 0x00000000; // Processor (default)
const ADC_EMUX_EM2_COMP0: u32 = 0x00000100; // Analog Comparator 0
const ADC_EMUX_EM2_COMP1: u32 = 0x00000200; // Analog Comparator 1
const ADC_EMUX_EM2_EXTERNAL: u32 = 0x00000400; // External (GPIO PB4)
const ADC_EMUX_EM2_TIMER: u32 = 0x00000500; // Timer
const ADC_EMUX_EM2_ALWAYS: u32 = 0x00000F00; // Always (continuously sample)
const ADC_EMUX_EM1_M: u32 = 0x000000F0; // SS1 Trigger Select
const ADC_EMUX_EM1_PROCESSOR: u32 = 0x00000000; // Processor (default)
const ADC_EMUX_EM1_COMP0: u32 = 0x00000010; // Analog Comparator 0
const ADC_EMUX_EM1_COMP1: u32 = 0x00000020; // Analog Comparator 1
const ADC_EMUX_EM1_EXTERNAL: u32 = 0x00000040; // External (GPIO PB4)
const ADC_EMUX_EM1_TIMER: u32 = 0x00000050; // Timer
const ADC_EMUX_EM1_ALWAYS: u32 = 0x000000F0; // Always (continuously sample)
const ADC_EMUX_EM0_M: u32 = 0x0000000F; // SS0 Trigger Select
const ADC_EMUX_EM0_PROCESSOR: u32 = 0x00000000; // Processor (default)
const ADC_EMUX_EM0_COMP0: u32 = 0x00000001; // Analog Comparator 0
const ADC_EMUX_EM0_COMP1: u32 = 0x00000002; // Analog Comparator 1
const ADC_EMUX_EM0_EXTERNAL: u32 = 0x00000004; // External (GPIO PB4)
const ADC_EMUX_EM0_TIMER: u32 = 0x00000005; // Timer
const ADC_EMUX_EM0_ALWAYS: u32 = 0x0000000F; // Always (continuously sample)

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_USTAT register.
//
// *****************************************************************************
const ADC_USTAT_UV3: u32 = 0x00000008; // SS3 FIFO Underflow
const ADC_USTAT_UV2: u32 = 0x00000004; // SS2 FIFO Underflow
const ADC_USTAT_UV1: u32 = 0x00000002; // SS1 FIFO Underflow
const ADC_USTAT_UV0: u32 = 0x00000001; // SS0 FIFO Underflow

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSPRI register.
//
// *****************************************************************************
const ADC_SSPRI_SS3_M: u32 = 0x00003000; // SS3 Priority
const ADC_SSPRI_SS3_1ST: u32 = 0x00000000; // First priority
const ADC_SSPRI_SS3_2ND: u32 = 0x00001000; // Second priority
const ADC_SSPRI_SS3_3RD: u32 = 0x00002000; // Third priority
const ADC_SSPRI_SS3_4TH: u32 = 0x00003000; // Fourth priority
const ADC_SSPRI_SS2_M: u32 = 0x00000300; // SS2 Priority
const ADC_SSPRI_SS2_1ST: u32 = 0x00000000; // First priority
const ADC_SSPRI_SS2_2ND: u32 = 0x00000100; // Second priority
const ADC_SSPRI_SS2_3RD: u32 = 0x00000200; // Third priority
const ADC_SSPRI_SS2_4TH: u32 = 0x00000300; // Fourth priority
const ADC_SSPRI_SS1_M: u32 = 0x00000030; // SS1 Priority
const ADC_SSPRI_SS1_1ST: u32 = 0x00000000; // First priority
const ADC_SSPRI_SS1_2ND: u32 = 0x00000010; // Second priority
const ADC_SSPRI_SS1_3RD: u32 = 0x00000020; // Third priority
const ADC_SSPRI_SS1_4TH: u32 = 0x00000030; // Fourth priority
const ADC_SSPRI_SS0_M: u32 = 0x00000003; // SS0 Priority
const ADC_SSPRI_SS0_1ST: u32 = 0x00000000; // First priority
const ADC_SSPRI_SS0_2ND: u32 = 0x00000001; // Second priority
const ADC_SSPRI_SS0_3RD: u32 = 0x00000002; // Third priority
const ADC_SSPRI_SS0_4TH: u32 = 0x00000003; // Fourth priority

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SPC register.
//
// *****************************************************************************
const ADC_SPC_PHASE_M: u32 = 0x0000000F; // Phase Difference
const ADC_SPC_PHASE_0: u32 = 0x00000000; // ADC sample lags by 0.0
const ADC_SPC_PHASE_22_5: u32 = 0x00000001; // ADC sample lags by 22.5
const ADC_SPC_PHASE_45: u32 = 0x00000002; // ADC sample lags by 45.0
const ADC_SPC_PHASE_67_5: u32 = 0x00000003; // ADC sample lags by 67.5
const ADC_SPC_PHASE_90: u32 = 0x00000004; // ADC sample lags by 90.0
const ADC_SPC_PHASE_112_5: u32 = 0x00000005; // ADC sample lags by 112.5
const ADC_SPC_PHASE_135: u32 = 0x00000006; // ADC sample lags by 135.0
const ADC_SPC_PHASE_157_5: u32 = 0x00000007; // ADC sample lags by 157.5
const ADC_SPC_PHASE_180: u32 = 0x00000008; // ADC sample lags by 180.0
const ADC_SPC_PHASE_202_5: u32 = 0x00000009; // ADC sample lags by 202.5
const ADC_SPC_PHASE_225: u32 = 0x0000000A; // ADC sample lags by 225.0
const ADC_SPC_PHASE_247_5: u32 = 0x0000000B; // ADC sample lags by 247.5
const ADC_SPC_PHASE_270: u32 = 0x0000000C; // ADC sample lags by 270.0
const ADC_SPC_PHASE_292_5: u32 = 0x0000000D; // ADC sample lags by 292.5
const ADC_SPC_PHASE_315: u32 = 0x0000000E; // ADC sample lags by 315.0
const ADC_SPC_PHASE_337_5: u32 = 0x0000000F; // ADC sample lags by 337.5

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_PSSI register.
//
// *****************************************************************************
const ADC_PSSI_GSYNC: u32 = 0x80000000; // Global Synchronize
const ADC_PSSI_SYNCWAIT: u32 = 0x08000000; // Synchronize Wait
const ADC_PSSI_SS3: u32 = 0x00000008; // SS3 Initiate
const ADC_PSSI_SS2: u32 = 0x00000004; // SS2 Initiate
const ADC_PSSI_SS1: u32 = 0x00000002; // SS1 Initiate
const ADC_PSSI_SS0: u32 = 0x00000001; // SS0 Initiate

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SAC register.
//
// *****************************************************************************
const ADC_SAC_AVG_M: u32 = 0x00000007; // Hardware Averaging Control
const ADC_SAC_AVG_OFF: u32 = 0x00000000; // No hardware oversampling
const ADC_SAC_AVG_2X: u32 = 0x00000001; // 2x hardware oversampling
const ADC_SAC_AVG_4X: u32 = 0x00000002; // 4x hardware oversampling
const ADC_SAC_AVG_8X: u32 = 0x00000003; // 8x hardware oversampling
const ADC_SAC_AVG_16X: u32 = 0x00000004; // 16x hardware oversampling
const ADC_SAC_AVG_32X: u32 = 0x00000005; // 32x hardware oversampling
const ADC_SAC_AVG_64X: u32 = 0x00000006; // 64x hardware oversampling

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCISC register.
//
// *****************************************************************************
const ADC_DCISC_DCINT7: u32 = 0x00000080; // Digital Comparator 7 Interrupt Status and Clear
const ADC_DCISC_DCINT6: u32 = 0x00000040; // Digital Comparator 6 Interrupt Status and Clear
const ADC_DCISC_DCINT5: u32 = 0x00000020; // Digital Comparator 5 Interrupt Status and Clear
const ADC_DCISC_DCINT4: u32 = 0x00000010; // Digital Comparator 4 Interrupt Status and Clear
const ADC_DCISC_DCINT3: u32 = 0x00000008; // Digital Comparator 3 Interrupt Status and Clear
const ADC_DCISC_DCINT2: u32 = 0x00000004; // Digital Comparator 2 Interrupt Status and Clear
const ADC_DCISC_DCINT1: u32 = 0x00000002; // Digital Comparator 1 Interrupt Status and Clear
const ADC_DCISC_DCINT0: u32 = 0x00000001; // Digital Comparator 0 Interrupt Status and Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSMUX0 register.
//
// *****************************************************************************
const ADC_SSMUX0_MUX7_M: u32 = 0xF0000000; // 8th Sample Input Select
const ADC_SSMUX0_MUX6_M: u32 = 0x0F000000; // 7th Sample Input Select
const ADC_SSMUX0_MUX5_M: u32 = 0x00F00000; // 6th Sample Input Select
const ADC_SSMUX0_MUX4_M: u32 = 0x000F0000; // 5th Sample Input Select
const ADC_SSMUX0_MUX3_M: u32 = 0x0000F000; // 4th Sample Input Select
const ADC_SSMUX0_MUX2_M: u32 = 0x00000F00; // 3rd Sample Input Select
const ADC_SSMUX0_MUX1_M: u32 = 0x000000F0; // 2nd Sample Input Select
const ADC_SSMUX0_MUX0_M: u32 = 0x0000000F; // 1st Sample Input Select
const ADC_SSMUX0_MUX7_S: u32 = 28;
const ADC_SSMUX0_MUX6_S: u32 = 24;
const ADC_SSMUX0_MUX5_S: u32 = 20;
const ADC_SSMUX0_MUX4_S: u32 = 16;
const ADC_SSMUX0_MUX3_S: u32 = 12;
const ADC_SSMUX0_MUX2_S: u32 = 8;
const ADC_SSMUX0_MUX1_S: u32 = 4;
const ADC_SSMUX0_MUX0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSCTL0 register.
//
// *****************************************************************************
const ADC_SSCTL0_TS7: u32 = 0x80000000; // 8th Sample Temp Sensor Select
const ADC_SSCTL0_IE7: u32 = 0x40000000; // 8th Sample Interrupt Enable
const ADC_SSCTL0_END7: u32 = 0x20000000; // 8th Sample is End of Sequence
const ADC_SSCTL0_D7: u32 = 0x10000000; // 8th Sample Diff Input Select
const ADC_SSCTL0_TS6: u32 = 0x08000000; // 7th Sample Temp Sensor Select
const ADC_SSCTL0_IE6: u32 = 0x04000000; // 7th Sample Interrupt Enable
const ADC_SSCTL0_END6: u32 = 0x02000000; // 7th Sample is End of Sequence
const ADC_SSCTL0_D6: u32 = 0x01000000; // 7th Sample Diff Input Select
const ADC_SSCTL0_TS5: u32 = 0x00800000; // 6th Sample Temp Sensor Select
const ADC_SSCTL0_IE5: u32 = 0x00400000; // 6th Sample Interrupt Enable
const ADC_SSCTL0_END5: u32 = 0x00200000; // 6th Sample is End of Sequence
const ADC_SSCTL0_D5: u32 = 0x00100000; // 6th Sample Diff Input Select
const ADC_SSCTL0_TS4: u32 = 0x00080000; // 5th Sample Temp Sensor Select
const ADC_SSCTL0_IE4: u32 = 0x00040000; // 5th Sample Interrupt Enable
const ADC_SSCTL0_END4: u32 = 0x00020000; // 5th Sample is End of Sequence
const ADC_SSCTL0_D4: u32 = 0x00010000; // 5th Sample Diff Input Select
const ADC_SSCTL0_TS3: u32 = 0x00008000; // 4th Sample Temp Sensor Select
const ADC_SSCTL0_IE3: u32 = 0x00004000; // 4th Sample Interrupt Enable
const ADC_SSCTL0_END3: u32 = 0x00002000; // 4th Sample is End of Sequence
const ADC_SSCTL0_D3: u32 = 0x00001000; // 4th Sample Diff Input Select
const ADC_SSCTL0_TS2: u32 = 0x00000800; // 3rd Sample Temp Sensor Select
const ADC_SSCTL0_IE2: u32 = 0x00000400; // 3rd Sample Interrupt Enable
const ADC_SSCTL0_END2: u32 = 0x00000200; // 3rd Sample is End of Sequence
const ADC_SSCTL0_D2: u32 = 0x00000100; // 3rd Sample Diff Input Select
const ADC_SSCTL0_TS1: u32 = 0x00000080; // 2nd Sample Temp Sensor Select
const ADC_SSCTL0_IE1: u32 = 0x00000040; // 2nd Sample Interrupt Enable
const ADC_SSCTL0_END1: u32 = 0x00000020; // 2nd Sample is End of Sequence
const ADC_SSCTL0_D1: u32 = 0x00000010; // 2nd Sample Diff Input Select
const ADC_SSCTL0_TS0: u32 = 0x00000008; // 1st Sample Temp Sensor Select
const ADC_SSCTL0_IE0: u32 = 0x00000004; // 1st Sample Interrupt Enable
const ADC_SSCTL0_END0: u32 = 0x00000002; // 1st Sample is End of Sequence
const ADC_SSCTL0_D0: u32 = 0x00000001; // 1st Sample Diff Input Select

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFIFO0 register.
//
// *****************************************************************************
const ADC_SSFIFO0_DATA_M: u32 = 0x00000FFF; // Conversion Result Data
const ADC_SSFIFO0_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFSTAT0 register.
//
// *****************************************************************************
const ADC_SSFSTAT0_FULL: u32 = 0x00001000; // FIFO Full
const ADC_SSFSTAT0_EMPTY: u32 = 0x00000100; // FIFO Empty
const ADC_SSFSTAT0_HPTR_M: u32 = 0x000000F0; // FIFO Head Pointer
const ADC_SSFSTAT0_TPTR_M: u32 = 0x0000000F; // FIFO Tail Pointer
const ADC_SSFSTAT0_HPTR_S: u32 = 4;
const ADC_SSFSTAT0_TPTR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSOP0 register.
//
// *****************************************************************************
const ADC_SSOP0_S7DCOP: u32 = 0x10000000; // Sample 7 Digital Comparator Operation
const ADC_SSOP0_S6DCOP: u32 = 0x01000000; // Sample 6 Digital Comparator Operation
const ADC_SSOP0_S5DCOP: u32 = 0x00100000; // Sample 5 Digital Comparator Operation
const ADC_SSOP0_S4DCOP: u32 = 0x00010000; // Sample 4 Digital Comparator Operation
const ADC_SSOP0_S3DCOP: u32 = 0x00001000; // Sample 3 Digital Comparator Operation
const ADC_SSOP0_S2DCOP: u32 = 0x00000100; // Sample 2 Digital Comparator Operation
const ADC_SSOP0_S1DCOP: u32 = 0x00000010; // Sample 1 Digital Comparator Operation
const ADC_SSOP0_S0DCOP: u32 = 0x00000001; // Sample 0 Digital Comparator Operation

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSDC0 register.
//
// *****************************************************************************
const ADC_SSDC0_S7DCSEL_M: u32 = 0xF0000000; // Sample 7 Digital Comparator Select
const ADC_SSDC0_S6DCSEL_M: u32 = 0x0F000000; // Sample 6 Digital Comparator Select
const ADC_SSDC0_S5DCSEL_M: u32 = 0x00F00000; // Sample 5 Digital Comparator Select
const ADC_SSDC0_S4DCSEL_M: u32 = 0x000F0000; // Sample 4 Digital Comparator Select
const ADC_SSDC0_S3DCSEL_M: u32 = 0x0000F000; // Sample 3 Digital Comparator Select
const ADC_SSDC0_S2DCSEL_M: u32 = 0x00000F00; // Sample 2 Digital Comparator Select
const ADC_SSDC0_S1DCSEL_M: u32 = 0x000000F0; // Sample 1 Digital Comparator Select
const ADC_SSDC0_S0DCSEL_M: u32 = 0x0000000F; // Sample 0 Digital Comparator Select
const ADC_SSDC0_S6DCSEL_S: u32 = 24;
const ADC_SSDC0_S5DCSEL_S: u32 = 20;
const ADC_SSDC0_S4DCSEL_S: u32 = 16;
const ADC_SSDC0_S3DCSEL_S: u32 = 12;
const ADC_SSDC0_S2DCSEL_S: u32 = 8;
const ADC_SSDC0_S1DCSEL_S: u32 = 4;
const ADC_SSDC0_S0DCSEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSMUX1 register.
//
// *****************************************************************************
const ADC_SSMUX1_MUX3_M: u32 = 0x0000F000; // 4th Sample Input Select
const ADC_SSMUX1_MUX2_M: u32 = 0x00000F00; // 3rd Sample Input Select
const ADC_SSMUX1_MUX1_M: u32 = 0x000000F0; // 2nd Sample Input Select
const ADC_SSMUX1_MUX0_M: u32 = 0x0000000F; // 1st Sample Input Select
const ADC_SSMUX1_MUX3_S: u32 = 12;
const ADC_SSMUX1_MUX2_S: u32 = 8;
const ADC_SSMUX1_MUX1_S: u32 = 4;
const ADC_SSMUX1_MUX0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSCTL1 register.
//
// *****************************************************************************
const ADC_SSCTL1_TS3: u32 = 0x00008000; // 4th Sample Temp Sensor Select
const ADC_SSCTL1_IE3: u32 = 0x00004000; // 4th Sample Interrupt Enable
const ADC_SSCTL1_END3: u32 = 0x00002000; // 4th Sample is End of Sequence
const ADC_SSCTL1_D3: u32 = 0x00001000; // 4th Sample Diff Input Select
const ADC_SSCTL1_TS2: u32 = 0x00000800; // 3rd Sample Temp Sensor Select
const ADC_SSCTL1_IE2: u32 = 0x00000400; // 3rd Sample Interrupt Enable
const ADC_SSCTL1_END2: u32 = 0x00000200; // 3rd Sample is End of Sequence
const ADC_SSCTL1_D2: u32 = 0x00000100; // 3rd Sample Diff Input Select
const ADC_SSCTL1_TS1: u32 = 0x00000080; // 2nd Sample Temp Sensor Select
const ADC_SSCTL1_IE1: u32 = 0x00000040; // 2nd Sample Interrupt Enable
const ADC_SSCTL1_END1: u32 = 0x00000020; // 2nd Sample is End of Sequence
const ADC_SSCTL1_D1: u32 = 0x00000010; // 2nd Sample Diff Input Select
const ADC_SSCTL1_TS0: u32 = 0x00000008; // 1st Sample Temp Sensor Select
const ADC_SSCTL1_IE0: u32 = 0x00000004; // 1st Sample Interrupt Enable
const ADC_SSCTL1_END0: u32 = 0x00000002; // 1st Sample is End of Sequence
const ADC_SSCTL1_D0: u32 = 0x00000001; // 1st Sample Diff Input Select

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFIFO1 register.
//
// *****************************************************************************
const ADC_SSFIFO1_DATA_M: u32 = 0x00000FFF; // Conversion Result Data
const ADC_SSFIFO1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFSTAT1 register.
//
// *****************************************************************************
const ADC_SSFSTAT1_FULL: u32 = 0x00001000; // FIFO Full
const ADC_SSFSTAT1_EMPTY: u32 = 0x00000100; // FIFO Empty
const ADC_SSFSTAT1_HPTR_M: u32 = 0x000000F0; // FIFO Head Pointer
const ADC_SSFSTAT1_TPTR_M: u32 = 0x0000000F; // FIFO Tail Pointer
const ADC_SSFSTAT1_HPTR_S: u32 = 4;
const ADC_SSFSTAT1_TPTR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSOP1 register.
//
// *****************************************************************************
const ADC_SSOP1_S3DCOP: u32 = 0x00001000; // Sample 3 Digital Comparator Operation
const ADC_SSOP1_S2DCOP: u32 = 0x00000100; // Sample 2 Digital Comparator Operation
const ADC_SSOP1_S1DCOP: u32 = 0x00000010; // Sample 1 Digital Comparator Operation
const ADC_SSOP1_S0DCOP: u32 = 0x00000001; // Sample 0 Digital Comparator Operation

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSDC1 register.
//
// *****************************************************************************
const ADC_SSDC1_S3DCSEL_M: u32 = 0x0000F000; // Sample 3 Digital Comparator Select
const ADC_SSDC1_S2DCSEL_M: u32 = 0x00000F00; // Sample 2 Digital Comparator Select
const ADC_SSDC1_S1DCSEL_M: u32 = 0x000000F0; // Sample 1 Digital Comparator Select
const ADC_SSDC1_S0DCSEL_M: u32 = 0x0000000F; // Sample 0 Digital Comparator Select
const ADC_SSDC1_S2DCSEL_S: u32 = 8;
const ADC_SSDC1_S1DCSEL_S: u32 = 4;
const ADC_SSDC1_S0DCSEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSMUX2 register.
//
// *****************************************************************************
const ADC_SSMUX2_MUX3_M: u32 = 0x0000F000; // 4th Sample Input Select
const ADC_SSMUX2_MUX2_M: u32 = 0x00000F00; // 3rd Sample Input Select
const ADC_SSMUX2_MUX1_M: u32 = 0x000000F0; // 2nd Sample Input Select
const ADC_SSMUX2_MUX0_M: u32 = 0x0000000F; // 1st Sample Input Select
const ADC_SSMUX2_MUX3_S: u32 = 12;
const ADC_SSMUX2_MUX2_S: u32 = 8;
const ADC_SSMUX2_MUX1_S: u32 = 4;
const ADC_SSMUX2_MUX0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSCTL2 register.
//
// *****************************************************************************
const ADC_SSCTL2_TS3: u32 = 0x00008000; // 4th Sample Temp Sensor Select
const ADC_SSCTL2_IE3: u32 = 0x00004000; // 4th Sample Interrupt Enable
const ADC_SSCTL2_END3: u32 = 0x00002000; // 4th Sample is End of Sequence
const ADC_SSCTL2_D3: u32 = 0x00001000; // 4th Sample Diff Input Select
const ADC_SSCTL2_TS2: u32 = 0x00000800; // 3rd Sample Temp Sensor Select
const ADC_SSCTL2_IE2: u32 = 0x00000400; // 3rd Sample Interrupt Enable
const ADC_SSCTL2_END2: u32 = 0x00000200; // 3rd Sample is End of Sequence
const ADC_SSCTL2_D2: u32 = 0x00000100; // 3rd Sample Diff Input Select
const ADC_SSCTL2_TS1: u32 = 0x00000080; // 2nd Sample Temp Sensor Select
const ADC_SSCTL2_IE1: u32 = 0x00000040; // 2nd Sample Interrupt Enable
const ADC_SSCTL2_END1: u32 = 0x00000020; // 2nd Sample is End of Sequence
const ADC_SSCTL2_D1: u32 = 0x00000010; // 2nd Sample Diff Input Select
const ADC_SSCTL2_TS0: u32 = 0x00000008; // 1st Sample Temp Sensor Select
const ADC_SSCTL2_IE0: u32 = 0x00000004; // 1st Sample Interrupt Enable
const ADC_SSCTL2_END0: u32 = 0x00000002; // 1st Sample is End of Sequence
const ADC_SSCTL2_D0: u32 = 0x00000001; // 1st Sample Diff Input Select

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFIFO2 register.
//
// *****************************************************************************
const ADC_SSFIFO2_DATA_M: u32 = 0x00000FFF; // Conversion Result Data
const ADC_SSFIFO2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFSTAT2 register.
//
// *****************************************************************************
const ADC_SSFSTAT2_FULL: u32 = 0x00001000; // FIFO Full
const ADC_SSFSTAT2_EMPTY: u32 = 0x00000100; // FIFO Empty
const ADC_SSFSTAT2_HPTR_M: u32 = 0x000000F0; // FIFO Head Pointer
const ADC_SSFSTAT2_TPTR_M: u32 = 0x0000000F; // FIFO Tail Pointer
const ADC_SSFSTAT2_HPTR_S: u32 = 4;
const ADC_SSFSTAT2_TPTR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSOP2 register.
//
// *****************************************************************************
const ADC_SSOP2_S3DCOP: u32 = 0x00001000; // Sample 3 Digital Comparator Operation
const ADC_SSOP2_S2DCOP: u32 = 0x00000100; // Sample 2 Digital Comparator Operation
const ADC_SSOP2_S1DCOP: u32 = 0x00000010; // Sample 1 Digital Comparator Operation
const ADC_SSOP2_S0DCOP: u32 = 0x00000001; // Sample 0 Digital Comparator Operation

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSDC2 register.
//
// *****************************************************************************
const ADC_SSDC2_S3DCSEL_M: u32 = 0x0000F000; // Sample 3 Digital Comparator Select
const ADC_SSDC2_S2DCSEL_M: u32 = 0x00000F00; // Sample 2 Digital Comparator Select
const ADC_SSDC2_S1DCSEL_M: u32 = 0x000000F0; // Sample 1 Digital Comparator Select
const ADC_SSDC2_S0DCSEL_M: u32 = 0x0000000F; // Sample 0 Digital Comparator Select
const ADC_SSDC2_S2DCSEL_S: u32 = 8;
const ADC_SSDC2_S1DCSEL_S: u32 = 4;
const ADC_SSDC2_S0DCSEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSMUX3 register.
//
// *****************************************************************************
const ADC_SSMUX3_MUX0_M: u32 = 0x0000000F; // 1st Sample Input Select
const ADC_SSMUX3_MUX0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSCTL3 register.
//
// *****************************************************************************
const ADC_SSCTL3_TS0: u32 = 0x00000008; // 1st Sample Temp Sensor Select
const ADC_SSCTL3_IE0: u32 = 0x00000004; // 1st Sample Interrupt Enable
const ADC_SSCTL3_END0: u32 = 0x00000002; // 1st Sample is End of Sequence
const ADC_SSCTL3_D0: u32 = 0x00000001; // 1st Sample Diff Input Select

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFIFO3 register.
//
// *****************************************************************************
const ADC_SSFIFO3_DATA_M: u32 = 0x00000FFF; // Conversion Result Data
const ADC_SSFIFO3_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSFSTAT3 register.
//
// *****************************************************************************
const ADC_SSFSTAT3_FULL: u32 = 0x00001000; // FIFO Full
const ADC_SSFSTAT3_EMPTY: u32 = 0x00000100; // FIFO Empty
const ADC_SSFSTAT3_HPTR_M: u32 = 0x000000F0; // FIFO Head Pointer
const ADC_SSFSTAT3_TPTR_M: u32 = 0x0000000F; // FIFO Tail Pointer
const ADC_SSFSTAT3_HPTR_S: u32 = 4;
const ADC_SSFSTAT3_TPTR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSOP3 register.
//
// *****************************************************************************
const ADC_SSOP3_S0DCOP: u32 = 0x00000001; // Sample 0 Digital Comparator Operation

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_SSDC3 register.
//
// *****************************************************************************
const ADC_SSDC3_S0DCSEL_M: u32 = 0x0000000F; // Sample 0 Digital Comparator Select

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCRIC register.
//
// *****************************************************************************
const ADC_DCRIC_DCTRIG7: u32 = 0x00800000; // Digital Comparator Trigger 7
const ADC_DCRIC_DCTRIG6: u32 = 0x00400000; // Digital Comparator Trigger 6
const ADC_DCRIC_DCTRIG5: u32 = 0x00200000; // Digital Comparator Trigger 5
const ADC_DCRIC_DCTRIG4: u32 = 0x00100000; // Digital Comparator Trigger 4
const ADC_DCRIC_DCTRIG3: u32 = 0x00080000; // Digital Comparator Trigger 3
const ADC_DCRIC_DCTRIG2: u32 = 0x00040000; // Digital Comparator Trigger 2
const ADC_DCRIC_DCTRIG1: u32 = 0x00020000; // Digital Comparator Trigger 1
const ADC_DCRIC_DCTRIG0: u32 = 0x00010000; // Digital Comparator Trigger 0
const ADC_DCRIC_DCINT7: u32 = 0x00000080; // Digital Comparator Interrupt 7
const ADC_DCRIC_DCINT6: u32 = 0x00000040; // Digital Comparator Interrupt 6
const ADC_DCRIC_DCINT5: u32 = 0x00000020; // Digital Comparator Interrupt 5
const ADC_DCRIC_DCINT4: u32 = 0x00000010; // Digital Comparator Interrupt 4
const ADC_DCRIC_DCINT3: u32 = 0x00000008; // Digital Comparator Interrupt 3
const ADC_DCRIC_DCINT2: u32 = 0x00000004; // Digital Comparator Interrupt 2
const ADC_DCRIC_DCINT1: u32 = 0x00000002; // Digital Comparator Interrupt 1
const ADC_DCRIC_DCINT0: u32 = 0x00000001; // Digital Comparator Interrupt 0

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL0 register.
//
// *****************************************************************************
const ADC_DCCTL0_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL0_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL0_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL0_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL0_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL0_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL0_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL0_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL0_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL0_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL1 register.
//
// *****************************************************************************
const ADC_DCCTL1_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL1_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL1_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL1_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL1_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL1_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL1_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL1_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL1_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL1_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL2 register.
//
// *****************************************************************************
const ADC_DCCTL2_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL2_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL2_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL2_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL2_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL2_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL2_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL2_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL2_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL2_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL3 register.
//
// *****************************************************************************
const ADC_DCCTL3_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL3_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL3_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL3_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL3_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL3_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL3_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL3_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL3_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL3_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL4 register.
//
// *****************************************************************************
const ADC_DCCTL4_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL4_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL4_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL4_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL4_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL4_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL4_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL4_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL4_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL4_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL5 register.
//
// *****************************************************************************
const ADC_DCCTL5_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL5_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL5_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL5_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL5_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL5_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL5_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL5_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL5_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL5_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL6 register.
//
// *****************************************************************************
const ADC_DCCTL6_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL6_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL6_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL6_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL6_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL6_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL6_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL6_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL6_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL6_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCTL7 register.
//
// *****************************************************************************
const ADC_DCCTL7_CIE: u32 = 0x00000010; // Comparison Interrupt Enable
const ADC_DCCTL7_CIC_M: u32 = 0x0000000C; // Comparison Interrupt Condition
const ADC_DCCTL7_CIC_LOW: u32 = 0x00000000; // Low Band
const ADC_DCCTL7_CIC_MID: u32 = 0x00000004; // Mid Band
const ADC_DCCTL7_CIC_HIGH: u32 = 0x0000000C; // High Band
const ADC_DCCTL7_CIM_M: u32 = 0x00000003; // Comparison Interrupt Mode
const ADC_DCCTL7_CIM_ALWAYS: u32 = 0x00000000; // Always
const ADC_DCCTL7_CIM_ONCE: u32 = 0x00000001; // Once
const ADC_DCCTL7_CIM_HALWAYS: u32 = 0x00000002; // Hysteresis Always
const ADC_DCCTL7_CIM_HONCE: u32 = 0x00000003; // Hysteresis Once

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP0 register.
//
// *****************************************************************************
const ADC_DCCMP0_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP0_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP0_COMP1_S: u32 = 16;
const ADC_DCCMP0_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP1 register.
//
// *****************************************************************************
const ADC_DCCMP1_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP1_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP1_COMP1_S: u32 = 16;
const ADC_DCCMP1_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP2 register.
//
// *****************************************************************************
const ADC_DCCMP2_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP2_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP2_COMP1_S: u32 = 16;
const ADC_DCCMP2_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP3 register.
//
// *****************************************************************************
const ADC_DCCMP3_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP3_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP3_COMP1_S: u32 = 16;
const ADC_DCCMP3_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP4 register.
//
// *****************************************************************************
const ADC_DCCMP4_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP4_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP4_COMP1_S: u32 = 16;
const ADC_DCCMP4_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP5 register.
//
// *****************************************************************************
const ADC_DCCMP5_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP5_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP5_COMP1_S: u32 = 16;
const ADC_DCCMP5_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP6 register.
//
// *****************************************************************************
const ADC_DCCMP6_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP6_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP6_COMP1_S: u32 = 16;
const ADC_DCCMP6_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_DCCMP7 register.
//
// *****************************************************************************
const ADC_DCCMP7_COMP1_M: u32 = 0x0FFF0000; // Compare 1
const ADC_DCCMP7_COMP0_M: u32 = 0x00000FFF; // Compare 0
const ADC_DCCMP7_COMP1_S: u32 = 16;
const ADC_DCCMP7_COMP0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_PP register.
//
// *****************************************************************************
const ADC_PP_TS: u32 = 0x00800000; // Temperature Sensor
const ADC_PP_RSL_M: u32 = 0x007C0000; // Resolution
const ADC_PP_TYPE_M: u32 = 0x00030000; // ADC Architecture
const ADC_PP_TYPE_SAR: u32 = 0x00000000; // SAR
const ADC_PP_DC_M: u32 = 0x0000FC00; // Digital Comparator Count
const ADC_PP_CH_M: u32 = 0x000003F0; // ADC Channel Count
const ADC_PP_MSR_M: u32 = 0x0000000F; // Maximum ADC Sample Rate
const ADC_PP_MSR_125K: u32 = 0x00000001; // 125 ksps
const ADC_PP_MSR_250K: u32 = 0x00000003; // 250 ksps
const ADC_PP_MSR_500K: u32 = 0x00000005; // 500 ksps
const ADC_PP_MSR_1M: u32 = 0x00000007; // 1 Msps
const ADC_PP_RSL_S: u32 = 18;
const ADC_PP_DC_S: u32 = 10;
const ADC_PP_CH_S: u32 = 4;

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_PC register.
//
// *****************************************************************************
const ADC_PC_SR_M: u32 = 0x0000000F; // ADC Sample Rate
const ADC_PC_SR_125K: u32 = 0x00000001; // 125 ksps
const ADC_PC_SR_250K: u32 = 0x00000003; // 250 ksps
const ADC_PC_SR_500K: u32 = 0x00000005; // 500 ksps
const ADC_PC_SR_1M: u32 = 0x00000007; // 1 Msps

// *****************************************************************************
//
// The following are defines for the bit fields in the ADC_O_CC register.
//
// *****************************************************************************
const ADC_CC_CS_M: u32 = 0x0000000F; // ADC Clock Source
const ADC_CC_CS_SYSPLL: u32 = 0x00000000; // Either the system clock (if the PLL bypass is in effect) or the 16 MHz clock derived from PLL / 25 (default)
const ADC_CC_CS_PIOSC: u32 = 0x00000001; // PIOSC

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACMIS register.
//
// *****************************************************************************
const COMP_ACMIS_IN1: u32 = 0x00000002; // Comparator 1 Masked Interrupt Status
const COMP_ACMIS_IN0: u32 = 0x00000001; // Comparator 0 Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACRIS register.
//
// *****************************************************************************
const COMP_ACRIS_IN1: u32 = 0x00000002; // Comparator 1 Interrupt Status
const COMP_ACRIS_IN0: u32 = 0x00000001; // Comparator 0 Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACINTEN register.
//
// *****************************************************************************
const COMP_ACINTEN_IN1: u32 = 0x00000002; // Comparator 1 Interrupt Enable
const COMP_ACINTEN_IN0: u32 = 0x00000001; // Comparator 0 Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACREFCTL
// register.
//
// *****************************************************************************
const COMP_ACREFCTL_EN: u32 = 0x00000200; // Resistor Ladder Enable
const COMP_ACREFCTL_RNG: u32 = 0x00000100; // Resistor Ladder Range
const COMP_ACREFCTL_VREF_M: u32 = 0x0000000F; // Resistor Ladder Voltage Ref
const COMP_ACREFCTL_VREF_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACSTAT0 register.
//
// *****************************************************************************
const COMP_ACSTAT0_OVAL: u32 = 0x00000002; // Comparator Output Value

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACCTL0 register.
//
// *****************************************************************************
const COMP_ACCTL0_TOEN: u32 = 0x00000800; // Trigger Output Enable
const COMP_ACCTL0_ASRCP_M: u32 = 0x00000600; // Analog Source Positive
const COMP_ACCTL0_ASRCP_PIN: u32 = 0x00000000; // Pin value of Cn+
const COMP_ACCTL0_ASRCP_PIN0: u32 = 0x00000200; // Pin value of C0+
const COMP_ACCTL0_ASRCP_REF: u32 = 0x00000400; // Internal voltage reference
const COMP_ACCTL0_TSLVAL: u32 = 0x00000080; // Trigger Sense Level Value
const COMP_ACCTL0_TSEN_M: u32 = 0x00000060; // Trigger Sense
const COMP_ACCTL0_TSEN_LEVEL: u32 = 0x00000000; // Level sense, see TSLVAL
const COMP_ACCTL0_TSEN_FALL: u32 = 0x00000020; // Falling edge
const COMP_ACCTL0_TSEN_RISE: u32 = 0x00000040; // Rising edge
const COMP_ACCTL0_TSEN_BOTH: u32 = 0x00000060; // Either edge
const COMP_ACCTL0_ISLVAL: u32 = 0x00000010; // Interrupt Sense Level Value
const COMP_ACCTL0_ISEN_M: u32 = 0x0000000C; // Interrupt Sense
const COMP_ACCTL0_ISEN_LEVEL: u32 = 0x00000000; // Level sense, see ISLVAL
const COMP_ACCTL0_ISEN_FALL: u32 = 0x00000004; // Falling edge
const COMP_ACCTL0_ISEN_RISE: u32 = 0x00000008; // Rising edge
const COMP_ACCTL0_ISEN_BOTH: u32 = 0x0000000C; // Either edge
const COMP_ACCTL0_CINV: u32 = 0x00000002; // Comparator Output Invert

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACSTAT1 register.
//
// *****************************************************************************
const COMP_ACSTAT1_OVAL: u32 = 0x00000002; // Comparator Output Value

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_ACCTL1 register.
//
// *****************************************************************************
const COMP_ACCTL1_TOEN: u32 = 0x00000800; // Trigger Output Enable
const COMP_ACCTL1_ASRCP_M: u32 = 0x00000600; // Analog Source Positive
const COMP_ACCTL1_ASRCP_PIN: u32 = 0x00000000; // Pin value of Cn+
const COMP_ACCTL1_ASRCP_PIN0: u32 = 0x00000200; // Pin value of C0+
const COMP_ACCTL1_ASRCP_REF: u32 = 0x00000400; // Internal voltage reference (VIREF)
const COMP_ACCTL1_TSLVAL: u32 = 0x00000080; // Trigger Sense Level Value
const COMP_ACCTL1_TSEN_M: u32 = 0x00000060; // Trigger Sense
const COMP_ACCTL1_TSEN_LEVEL: u32 = 0x00000000; // Level sense, see TSLVAL
const COMP_ACCTL1_TSEN_FALL: u32 = 0x00000020; // Falling edge
const COMP_ACCTL1_TSEN_RISE: u32 = 0x00000040; // Rising edge
const COMP_ACCTL1_TSEN_BOTH: u32 = 0x00000060; // Either edge
const COMP_ACCTL1_ISLVAL: u32 = 0x00000010; // Interrupt Sense Level Value
const COMP_ACCTL1_ISEN_M: u32 = 0x0000000C; // Interrupt Sense
const COMP_ACCTL1_ISEN_LEVEL: u32 = 0x00000000; // Level sense, see ISLVAL
const COMP_ACCTL1_ISEN_FALL: u32 = 0x00000004; // Falling edge
const COMP_ACCTL1_ISEN_RISE: u32 = 0x00000008; // Rising edge
const COMP_ACCTL1_ISEN_BOTH: u32 = 0x0000000C; // Either edge
const COMP_ACCTL1_CINV: u32 = 0x00000002; // Comparator Output Invert

// *****************************************************************************
//
// The following are defines for the bit fields in the COMP_O_PP register.
//
// *****************************************************************************
const COMP_PP_C2O: u32 = 0x00040000; // Comparator Output 2 Present
const COMP_PP_C1O: u32 = 0x00020000; // Comparator Output 1 Present
const COMP_PP_C0O: u32 = 0x00010000; // Comparator Output 0 Present
const COMP_PP_CMP2: u32 = 0x00000004; // Comparator 2 Present
const COMP_PP_CMP1: u32 = 0x00000002; // Comparator 1 Present
const COMP_PP_CMP0: u32 = 0x00000001; // Comparator 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_CTL register.
//
// *****************************************************************************
const CAN_CTL_TEST: u32 = 0x00000080; // Test Mode Enable
const CAN_CTL_CCE: u32 = 0x00000040; // Configuration Change Enable
const CAN_CTL_DAR: u32 = 0x00000020; // Disable Automatic-Retransmission
const CAN_CTL_EIE: u32 = 0x00000008; // Error Interrupt Enable
const CAN_CTL_SIE: u32 = 0x00000004; // Status Interrupt Enable
const CAN_CTL_IE: u32 = 0x00000002; // CAN Interrupt Enable
const CAN_CTL_INIT: u32 = 0x00000001; // Initialization

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_STS register.
//
// *****************************************************************************
const CAN_STS_BOFF: u32 = 0x00000080; // Bus-Off Status
const CAN_STS_EWARN: u32 = 0x00000040; // Warning Status
const CAN_STS_EPASS: u32 = 0x00000020; // Error Passive
const CAN_STS_RXOK: u32 = 0x00000010; // Received a Message Successfully
const CAN_STS_TXOK: u32 = 0x00000008; // Transmitted a Message Successfully
const CAN_STS_LEC_M: u32 = 0x00000007; // Last Error Code
const CAN_STS_LEC_NONE: u32 = 0x00000000; // No Error
const CAN_STS_LEC_STUFF: u32 = 0x00000001; // Stuff Error
const CAN_STS_LEC_FORM: u32 = 0x00000002; // Format Error
const CAN_STS_LEC_ACK: u32 = 0x00000003; // ACK Error
const CAN_STS_LEC_BIT1: u32 = 0x00000004; // Bit 1 Error
const CAN_STS_LEC_BIT0: u32 = 0x00000005; // Bit 0 Error
const CAN_STS_LEC_CRC: u32 = 0x00000006; // CRC Error
const CAN_STS_LEC_NOEVENT: u32 = 0x00000007; // No Event

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_ERR register.
//
// *****************************************************************************
const CAN_ERR_RP: u32 = 0x00008000; // Received Error Passive
const CAN_ERR_REC_M: u32 = 0x00007F00; // Receive Error Counter
const CAN_ERR_TEC_M: u32 = 0x000000FF; // Transmit Error Counter
const CAN_ERR_REC_S: u32 = 8;
const CAN_ERR_TEC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_BIT register.
//
// *****************************************************************************
const CAN_BIT_TSEG2_M: u32 = 0x00007000; // Time Segment after Sample Point
const CAN_BIT_TSEG1_M: u32 = 0x00000F00; // Time Segment Before Sample Point
const CAN_BIT_SJW_M: u32 = 0x000000C0; // (Re)Synchronization Jump Width
const CAN_BIT_BRP_M: u32 = 0x0000003F; // Baud Rate Prescaler
const CAN_BIT_TSEG2_S: u32 = 12;
const CAN_BIT_TSEG1_S: u32 = 8;
const CAN_BIT_SJW_S: u32 = 6;
const CAN_BIT_BRP_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_INT register.
//
// *****************************************************************************
const CAN_INT_INTID_M: u32 = 0x0000FFFF; // Interrupt Identifier
const CAN_INT_INTID_NONE: u32 = 0x00000000; // No interrupt pending
const CAN_INT_INTID_STATUS: u32 = 0x00008000; // Status Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_TST register.
//
// *****************************************************************************
const CAN_TST_RX: u32 = 0x00000080; // Receive Observation
const CAN_TST_TX_M: u32 = 0x00000060; // Transmit Control
const CAN_TST_TX_CANCTL: u32 = 0x00000000; // CAN Module Control
const CAN_TST_TX_SAMPLE: u32 = 0x00000020; // Sample Point
const CAN_TST_TX_DOMINANT: u32 = 0x00000040; // Driven Low
const CAN_TST_TX_RECESSIVE: u32 = 0x00000060; // Driven High
const CAN_TST_LBACK: u32 = 0x00000010; // Loopback Mode
const CAN_TST_SILENT: u32 = 0x00000008; // Silent Mode
const CAN_TST_BASIC: u32 = 0x00000004; // Basic Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_BRPE register.
//
// *****************************************************************************
const CAN_BRPE_BRPE_M: u32 = 0x0000000F; // Baud Rate Prescaler Extension
const CAN_BRPE_BRPE_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1CRQ register.
//
// *****************************************************************************
const CAN_IF1CRQ_BUSY: u32 = 0x00008000; // Busy Flag
const CAN_IF1CRQ_MNUM_M: u32 = 0x0000003F; // Message Number
const CAN_IF1CRQ_MNUM_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1CMSK register.
//
// *****************************************************************************
const CAN_IF1CMSK_WRNRD: u32 = 0x00000080; // Write, Not Read
const CAN_IF1CMSK_MASK: u32 = 0x00000040; // Access Mask Bits
const CAN_IF1CMSK_ARB: u32 = 0x00000020; // Access Arbitration Bits
const CAN_IF1CMSK_CONTROL: u32 = 0x00000010; // Access Control Bits
const CAN_IF1CMSK_CLRINTPND: u32 = 0x00000008; // Clear Interrupt Pending Bit
const CAN_IF1CMSK_NEWDAT: u32 = 0x00000004; // Access New Data
const CAN_IF1CMSK_TXRQST: u32 = 0x00000004; // Access Transmission Request
const CAN_IF1CMSK_DATAA: u32 = 0x00000002; // Access Data Byte 0 to 3
const CAN_IF1CMSK_DATAB: u32 = 0x00000001; // Access Data Byte 4 to 7

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1MSK1 register.
//
// *****************************************************************************
const CAN_IF1MSK1_IDMSK_M: u32 = 0x0000FFFF; // Identifier Mask
const CAN_IF1MSK1_IDMSK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1MSK2 register.
//
// *****************************************************************************
const CAN_IF1MSK2_MXTD: u32 = 0x00008000; // Mask Extended Identifier
const CAN_IF1MSK2_MDIR: u32 = 0x00004000; // Mask Message Direction
const CAN_IF1MSK2_IDMSK_M: u32 = 0x00001FFF; // Identifier Mask
const CAN_IF1MSK2_IDMSK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1ARB1 register.
//
// *****************************************************************************
const CAN_IF1ARB1_ID_M: u32 = 0x0000FFFF; // Message Identifier
const CAN_IF1ARB1_ID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1ARB2 register.
//
// *****************************************************************************
const CAN_IF1ARB2_MSGVAL: u32 = 0x00008000; // Message Valid
const CAN_IF1ARB2_XTD: u32 = 0x00004000; // Extended Identifier
const CAN_IF1ARB2_DIR: u32 = 0x00002000; // Message Direction
const CAN_IF1ARB2_ID_M: u32 = 0x00001FFF; // Message Identifier
const CAN_IF1ARB2_ID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1MCTL register.
//
// *****************************************************************************
const CAN_IF1MCTL_NEWDAT: u32 = 0x00008000; // New Data
const CAN_IF1MCTL_MSGLST: u32 = 0x00004000; // Message Lost
const CAN_IF1MCTL_INTPND: u32 = 0x00002000; // Interrupt Pending
const CAN_IF1MCTL_UMASK: u32 = 0x00001000; // Use Acceptance Mask
const CAN_IF1MCTL_TXIE: u32 = 0x00000800; // Transmit Interrupt Enable
const CAN_IF1MCTL_RXIE: u32 = 0x00000400; // Receive Interrupt Enable
const CAN_IF1MCTL_RMTEN: u32 = 0x00000200; // Remote Enable
const CAN_IF1MCTL_TXRQST: u32 = 0x00000100; // Transmit Request
const CAN_IF1MCTL_EOB: u32 = 0x00000080; // End of Buffer
const CAN_IF1MCTL_DLC_M: u32 = 0x0000000F; // Data Length Code
const CAN_IF1MCTL_DLC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1DA1 register.
//
// *****************************************************************************
const CAN_IF1DA1_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF1DA1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1DA2 register.
//
// *****************************************************************************
const CAN_IF1DA2_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF1DA2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1DB1 register.
//
// *****************************************************************************
const CAN_IF1DB1_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF1DB1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF1DB2 register.
//
// *****************************************************************************
const CAN_IF1DB2_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF1DB2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2CRQ register.
//
// *****************************************************************************
const CAN_IF2CRQ_BUSY: u32 = 0x00008000; // Busy Flag
const CAN_IF2CRQ_MNUM_M: u32 = 0x0000003F; // Message Number
const CAN_IF2CRQ_MNUM_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2CMSK register.
//
// *****************************************************************************
const CAN_IF2CMSK_WRNRD: u32 = 0x00000080; // Write, Not Read
const CAN_IF2CMSK_MASK: u32 = 0x00000040; // Access Mask Bits
const CAN_IF2CMSK_ARB: u32 = 0x00000020; // Access Arbitration Bits
const CAN_IF2CMSK_CONTROL: u32 = 0x00000010; // Access Control Bits
const CAN_IF2CMSK_CLRINTPND: u32 = 0x00000008; // Clear Interrupt Pending Bit
const CAN_IF2CMSK_NEWDAT: u32 = 0x00000004; // Access New Data
const CAN_IF2CMSK_TXRQST: u32 = 0x00000004; // Access Transmission Request
const CAN_IF2CMSK_DATAA: u32 = 0x00000002; // Access Data Byte 0 to 3
const CAN_IF2CMSK_DATAB: u32 = 0x00000001; // Access Data Byte 4 to 7

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2MSK1 register.
//
// *****************************************************************************
const CAN_IF2MSK1_IDMSK_M: u32 = 0x0000FFFF; // Identifier Mask
const CAN_IF2MSK1_IDMSK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2MSK2 register.
//
// *****************************************************************************
const CAN_IF2MSK2_MXTD: u32 = 0x00008000; // Mask Extended Identifier
const CAN_IF2MSK2_MDIR: u32 = 0x00004000; // Mask Message Direction
const CAN_IF2MSK2_IDMSK_M: u32 = 0x00001FFF; // Identifier Mask
const CAN_IF2MSK2_IDMSK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2ARB1 register.
//
// *****************************************************************************
const CAN_IF2ARB1_ID_M: u32 = 0x0000FFFF; // Message Identifier
const CAN_IF2ARB1_ID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2ARB2 register.
//
// *****************************************************************************
const CAN_IF2ARB2_MSGVAL: u32 = 0x00008000; // Message Valid
const CAN_IF2ARB2_XTD: u32 = 0x00004000; // Extended Identifier
const CAN_IF2ARB2_DIR: u32 = 0x00002000; // Message Direction
const CAN_IF2ARB2_ID_M: u32 = 0x00001FFF; // Message Identifier
const CAN_IF2ARB2_ID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2MCTL register.
//
// *****************************************************************************
const CAN_IF2MCTL_NEWDAT: u32 = 0x00008000; // New Data
const CAN_IF2MCTL_MSGLST: u32 = 0x00004000; // Message Lost
const CAN_IF2MCTL_INTPND: u32 = 0x00002000; // Interrupt Pending
const CAN_IF2MCTL_UMASK: u32 = 0x00001000; // Use Acceptance Mask
const CAN_IF2MCTL_TXIE: u32 = 0x00000800; // Transmit Interrupt Enable
const CAN_IF2MCTL_RXIE: u32 = 0x00000400; // Receive Interrupt Enable
const CAN_IF2MCTL_RMTEN: u32 = 0x00000200; // Remote Enable
const CAN_IF2MCTL_TXRQST: u32 = 0x00000100; // Transmit Request
const CAN_IF2MCTL_EOB: u32 = 0x00000080; // End of Buffer
const CAN_IF2MCTL_DLC_M: u32 = 0x0000000F; // Data Length Code
const CAN_IF2MCTL_DLC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2DA1 register.
//
// *****************************************************************************
const CAN_IF2DA1_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF2DA1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2DA2 register.
//
// *****************************************************************************
const CAN_IF2DA2_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF2DA2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2DB1 register.
//
// *****************************************************************************
const CAN_IF2DB1_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF2DB1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_IF2DB2 register.
//
// *****************************************************************************
const CAN_IF2DB2_DATA_M: u32 = 0x0000FFFF; // Data
const CAN_IF2DB2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_TXRQ1 register.
//
// *****************************************************************************
const CAN_TXRQ1_TXRQST_M: u32 = 0x0000FFFF; // Transmission Request Bits
const CAN_TXRQ1_TXRQST_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_TXRQ2 register.
//
// *****************************************************************************
const CAN_TXRQ2_TXRQST_M: u32 = 0x0000FFFF; // Transmission Request Bits
const CAN_TXRQ2_TXRQST_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_NWDA1 register.
//
// *****************************************************************************
const CAN_NWDA1_NEWDAT_M: u32 = 0x0000FFFF; // New Data Bits
const CAN_NWDA1_NEWDAT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_NWDA2 register.
//
// *****************************************************************************
const CAN_NWDA2_NEWDAT_M: u32 = 0x0000FFFF; // New Data Bits
const CAN_NWDA2_NEWDAT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_MSG1INT register.
//
// *****************************************************************************
const CAN_MSG1INT_INTPND_M: u32 = 0x0000FFFF; // Interrupt Pending Bits
const CAN_MSG1INT_INTPND_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_MSG2INT register.
//
// *****************************************************************************
const CAN_MSG2INT_INTPND_M: u32 = 0x0000FFFF; // Interrupt Pending Bits
const CAN_MSG2INT_INTPND_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_MSG1VAL register.
//
// *****************************************************************************
const CAN_MSG1VAL_MSGVAL_M: u32 = 0x0000FFFF; // Message Valid Bits
const CAN_MSG1VAL_MSGVAL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the CAN_O_MSG2VAL register.
//
// *****************************************************************************
const CAN_MSG2VAL_MSGVAL_M: u32 = 0x0000FFFF; // Message Valid Bits
const CAN_MSG2VAL_MSGVAL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FADDR register.
//
// *****************************************************************************
const USB_FADDR_M: u32 = 0x0000007F; // Function Address
const USB_FADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_POWER register.
//
// *****************************************************************************
const USB_POWER_ISOUP: u32 = 0x00000080; // Isochronous Update
const USB_POWER_SOFTCONN: u32 = 0x00000040; // Soft Connect/Disconnect
const USB_POWER_RESET: u32 = 0x00000008; // RESET Signaling
const USB_POWER_RESUME: u32 = 0x00000004; // RESUME Signaling
const USB_POWER_SUSPEND: u32 = 0x00000002; // SUSPEND Mode
const USB_POWER_PWRDNPHY: u32 = 0x00000001; // Power Down PHY

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXIS register.
//
// *****************************************************************************
const USB_TXIS_EP7: u32 = 0x00000080; // TX Endpoint 7 Interrupt
const USB_TXIS_EP6: u32 = 0x00000040; // TX Endpoint 6 Interrupt
const USB_TXIS_EP5: u32 = 0x00000020; // TX Endpoint 5 Interrupt
const USB_TXIS_EP4: u32 = 0x00000010; // TX Endpoint 4 Interrupt
const USB_TXIS_EP3: u32 = 0x00000008; // TX Endpoint 3 Interrupt
const USB_TXIS_EP2: u32 = 0x00000004; // TX Endpoint 2 Interrupt
const USB_TXIS_EP1: u32 = 0x00000002; // TX Endpoint 1 Interrupt
const USB_TXIS_EP0: u32 = 0x00000001; // TX and RX Endpoint 0 Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXIS register.
//
// *****************************************************************************
const USB_RXIS_EP7: u32 = 0x00000080; // RX Endpoint 7 Interrupt
const USB_RXIS_EP6: u32 = 0x00000040; // RX Endpoint 6 Interrupt
const USB_RXIS_EP5: u32 = 0x00000020; // RX Endpoint 5 Interrupt
const USB_RXIS_EP4: u32 = 0x00000010; // RX Endpoint 4 Interrupt
const USB_RXIS_EP3: u32 = 0x00000008; // RX Endpoint 3 Interrupt
const USB_RXIS_EP2: u32 = 0x00000004; // RX Endpoint 2 Interrupt
const USB_RXIS_EP1: u32 = 0x00000002; // RX Endpoint 1 Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXIE register.
//
// *****************************************************************************
const USB_TXIE_EP7: u32 = 0x00000080; // TX Endpoint 7 Interrupt Enable
const USB_TXIE_EP6: u32 = 0x00000040; // TX Endpoint 6 Interrupt Enable
const USB_TXIE_EP5: u32 = 0x00000020; // TX Endpoint 5 Interrupt Enable
const USB_TXIE_EP4: u32 = 0x00000010; // TX Endpoint 4 Interrupt Enable
const USB_TXIE_EP3: u32 = 0x00000008; // TX Endpoint 3 Interrupt Enable
const USB_TXIE_EP2: u32 = 0x00000004; // TX Endpoint 2 Interrupt Enable
const USB_TXIE_EP1: u32 = 0x00000002; // TX Endpoint 1 Interrupt Enable
const USB_TXIE_EP0: u32 = 0x00000001; // TX and RX Endpoint 0 Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXIE register.
//
// *****************************************************************************
const USB_RXIE_EP7: u32 = 0x00000080; // RX Endpoint 7 Interrupt Enable
const USB_RXIE_EP6: u32 = 0x00000040; // RX Endpoint 6 Interrupt Enable
const USB_RXIE_EP5: u32 = 0x00000020; // RX Endpoint 5 Interrupt Enable
const USB_RXIE_EP4: u32 = 0x00000010; // RX Endpoint 4 Interrupt Enable
const USB_RXIE_EP3: u32 = 0x00000008; // RX Endpoint 3 Interrupt Enable
const USB_RXIE_EP2: u32 = 0x00000004; // RX Endpoint 2 Interrupt Enable
const USB_RXIE_EP1: u32 = 0x00000002; // RX Endpoint 1 Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IS register.
//
// *****************************************************************************
const USB_IS_DISCON: u32 = 0x00000020; // Session Disconnect
const USB_IS_SOF: u32 = 0x00000008; // Start of Frame
const USB_IS_RESET: u32 = 0x00000004; // RESET Signaling Detected
const USB_IS_RESUME: u32 = 0x00000002; // RESUME Signaling Detected
const USB_IS_SUSPEND: u32 = 0x00000001; // SUSPEND Signaling Detected

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_IE register.
//
// *****************************************************************************
const USB_IE_DISCON: u32 = 0x00000020; // Enable Disconnect Interrupt
const USB_IE_SOF: u32 = 0x00000008; // Enable Start-of-Frame Interrupt
const USB_IE_RESET: u32 = 0x00000004; // Enable RESET Interrupt
const USB_IE_RESUME: u32 = 0x00000002; // Enable RESUME Interrupt
const USB_IE_SUSPND: u32 = 0x00000001; // Enable SUSPEND Interrupt

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FRAME register.
//
// *****************************************************************************
const USB_FRAME_M: u32 = 0x000007FF; // Frame Number
const USB_FRAME_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_EPIDX register.
//
// *****************************************************************************
const USB_EPIDX_EPIDX_M: u32 = 0x0000000F; // Endpoint Index
const USB_EPIDX_EPIDX_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TEST register.
//
// *****************************************************************************
const USB_TEST_FIFOACC: u32 = 0x00000040; // FIFO Access
const USB_TEST_FORCEFS: u32 = 0x00000020; // Force Full-Speed Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO0 register.
//
// *****************************************************************************
const USB_FIFO0_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO0_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO1 register.
//
// *****************************************************************************
const USB_FIFO1_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO1_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO2 register.
//
// *****************************************************************************
const USB_FIFO2_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO2_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO3 register.
//
// *****************************************************************************
const USB_FIFO3_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO3_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO4 register.
//
// *****************************************************************************
const USB_FIFO4_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO4_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO5 register.
//
// *****************************************************************************
const USB_FIFO5_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO5_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO6 register.
//
// *****************************************************************************
const USB_FIFO6_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO6_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FIFO7 register.
//
// *****************************************************************************
const USB_FIFO7_EPDATA_M: u32 = 0xFFFFFFFF; // Endpoint Data
const USB_FIFO7_EPDATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFIFOSZ register.
//
// *****************************************************************************
const USB_TXFIFOSZ_DPB: u32 = 0x00000010; // Double Packet Buffer Support
const USB_TXFIFOSZ_SIZE_M: u32 = 0x0000000F; // Max Packet Size
const USB_TXFIFOSZ_SIZE_8: u32 = 0x00000000; // 8
const USB_TXFIFOSZ_SIZE_16: u32 = 0x00000001; // 16
const USB_TXFIFOSZ_SIZE_32: u32 = 0x00000002; // 32
const USB_TXFIFOSZ_SIZE_64: u32 = 0x00000003; // 64
const USB_TXFIFOSZ_SIZE_128: u32 = 0x00000004; // 128
const USB_TXFIFOSZ_SIZE_256: u32 = 0x00000005; // 256
const USB_TXFIFOSZ_SIZE_512: u32 = 0x00000006; // 512
const USB_TXFIFOSZ_SIZE_1024: u32 = 0x00000007; // 1024
const USB_TXFIFOSZ_SIZE_2048: u32 = 0x00000008; // 2048

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFIFOSZ register.
//
// *****************************************************************************
const USB_RXFIFOSZ_DPB: u32 = 0x00000010; // Double Packet Buffer Support
const USB_RXFIFOSZ_SIZE_M: u32 = 0x0000000F; // Max Packet Size
const USB_RXFIFOSZ_SIZE_8: u32 = 0x00000000; // 8
const USB_RXFIFOSZ_SIZE_16: u32 = 0x00000001; // 16
const USB_RXFIFOSZ_SIZE_32: u32 = 0x00000002; // 32
const USB_RXFIFOSZ_SIZE_64: u32 = 0x00000003; // 64
const USB_RXFIFOSZ_SIZE_128: u32 = 0x00000004; // 128
const USB_RXFIFOSZ_SIZE_256: u32 = 0x00000005; // 256
const USB_RXFIFOSZ_SIZE_512: u32 = 0x00000006; // 512
const USB_RXFIFOSZ_SIZE_1024: u32 = 0x00000007; // 1024
const USB_RXFIFOSZ_SIZE_2048: u32 = 0x00000008; // 2048

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXFIFOADD
// register.
//
// *****************************************************************************
const USB_TXFIFOADD_ADDR_M: u32 = 0x000001FF; // Transmit/Receive Start Address
const USB_TXFIFOADD_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXFIFOADD
// register.
//
// *****************************************************************************
const USB_RXFIFOADD_ADDR_M: u32 = 0x000001FF; // Transmit/Receive Start Address
const USB_RXFIFOADD_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CONTIM register.
//
// *****************************************************************************
const USB_CONTIM_WTCON_M: u32 = 0x000000F0; // Connect Wait
const USB_CONTIM_WTID_M: u32 = 0x0000000F; // Wait ID
const USB_CONTIM_WTCON_S: u32 = 4;
const USB_CONTIM_WTID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_FSEOF register.
//
// *****************************************************************************
const USB_FSEOF_FSEOFG_M: u32 = 0x000000FF; // Full-Speed End-of-Frame Gap
const USB_FSEOF_FSEOFG_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_LSEOF register.
//
// *****************************************************************************
const USB_LSEOF_LSEOFG_M: u32 = 0x000000FF; // Low-Speed End-of-Frame Gap
const USB_LSEOF_LSEOFG_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CSRL0 register.
//
// *****************************************************************************
const USB_CSRL0_SETENDC: u32 = 0x00000080; // Setup End Clear
const USB_CSRL0_RXRDYC: u32 = 0x00000040; // RXRDY Clear
const USB_CSRL0_STALL: u32 = 0x00000020; // Send Stall
const USB_CSRL0_SETEND: u32 = 0x00000010; // Setup End
const USB_CSRL0_DATAEND: u32 = 0x00000008; // Data End
const USB_CSRL0_STALLED: u32 = 0x00000004; // Endpoint Stalled
const USB_CSRL0_TXRDY: u32 = 0x00000002; // Transmit Packet Ready
const USB_CSRL0_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_CSRH0 register.
//
// *****************************************************************************
const USB_CSRH0_FLUSH: u32 = 0x00000001; // Flush FIFO

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_COUNT0 register.
//
// *****************************************************************************
const USB_COUNT0_COUNT_M: u32 = 0x0000007F; // FIFO Count
const USB_COUNT0_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP1 register.
//
// *****************************************************************************
const USB_TXMAXP1_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP1_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL1 register.
//
// *****************************************************************************
const USB_TXCSRL1_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL1_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL1_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL1_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL1_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL1_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL1_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH1 register.
//
// *****************************************************************************
const USB_TXCSRH1_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH1_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH1_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH1_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH1_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH1_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP1 register.
//
// *****************************************************************************
const USB_RXMAXP1_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP1_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL1 register.
//
// *****************************************************************************
const USB_RXCSRL1_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL1_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL1_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL1_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL1_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL1_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL1_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL1_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH1 register.
//
// *****************************************************************************
const USB_RXCSRH1_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH1_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH1_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH1_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH1_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH1_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT1 register.
//
// *****************************************************************************
const USB_RXCOUNT1_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT1_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP2 register.
//
// *****************************************************************************
const USB_TXMAXP2_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP2_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL2 register.
//
// *****************************************************************************
const USB_TXCSRL2_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL2_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL2_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL2_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL2_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL2_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL2_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH2 register.
//
// *****************************************************************************
const USB_TXCSRH2_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH2_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH2_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH2_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH2_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH2_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP2 register.
//
// *****************************************************************************
const USB_RXMAXP2_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP2_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL2 register.
//
// *****************************************************************************
const USB_RXCSRL2_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL2_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL2_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL2_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL2_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL2_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL2_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL2_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH2 register.
//
// *****************************************************************************
const USB_RXCSRH2_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH2_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH2_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH2_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH2_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH2_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT2 register.
//
// *****************************************************************************
const USB_RXCOUNT2_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT2_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP3 register.
//
// *****************************************************************************
const USB_TXMAXP3_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP3_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL3 register.
//
// *****************************************************************************
const USB_TXCSRL3_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL3_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL3_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL3_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL3_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL3_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL3_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH3 register.
//
// *****************************************************************************
const USB_TXCSRH3_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH3_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH3_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH3_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH3_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH3_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP3 register.
//
// *****************************************************************************
const USB_RXMAXP3_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP3_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL3 register.
//
// *****************************************************************************
const USB_RXCSRL3_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL3_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL3_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL3_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL3_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL3_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL3_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL3_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH3 register.
//
// *****************************************************************************
const USB_RXCSRH3_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH3_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH3_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH3_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH3_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH3_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT3 register.
//
// *****************************************************************************
const USB_RXCOUNT3_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT3_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP4 register.
//
// *****************************************************************************
const USB_TXMAXP4_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP4_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL4 register.
//
// *****************************************************************************
const USB_TXCSRL4_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL4_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL4_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL4_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL4_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL4_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL4_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH4 register.
//
// *****************************************************************************
const USB_TXCSRH4_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH4_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH4_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH4_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH4_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH4_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP4 register.
//
// *****************************************************************************
const USB_RXMAXP4_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP4_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL4 register.
//
// *****************************************************************************
const USB_RXCSRL4_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL4_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL4_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL4_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL4_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL4_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL4_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL4_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH4 register.
//
// *****************************************************************************
const USB_RXCSRH4_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH4_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH4_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH4_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH4_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH4_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT4 register.
//
// *****************************************************************************
const USB_RXCOUNT4_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT4_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP5 register.
//
// *****************************************************************************
const USB_TXMAXP5_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP5_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL5 register.
//
// *****************************************************************************
const USB_TXCSRL5_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL5_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL5_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL5_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL5_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL5_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL5_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH5 register.
//
// *****************************************************************************
const USB_TXCSRH5_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH5_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH5_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH5_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH5_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH5_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP5 register.
//
// *****************************************************************************
const USB_RXMAXP5_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP5_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL5 register.
//
// *****************************************************************************
const USB_RXCSRL5_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL5_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL5_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL5_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL5_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL5_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL5_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL5_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH5 register.
//
// *****************************************************************************
const USB_RXCSRH5_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH5_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH5_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH5_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH5_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH5_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT5 register.
//
// *****************************************************************************
const USB_RXCOUNT5_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT5_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP6 register.
//
// *****************************************************************************
const USB_TXMAXP6_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP6_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL6 register.
//
// *****************************************************************************
const USB_TXCSRL6_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL6_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL6_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL6_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL6_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL6_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL6_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH6 register.
//
// *****************************************************************************
const USB_TXCSRH6_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH6_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH6_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH6_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH6_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH6_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP6 register.
//
// *****************************************************************************
const USB_RXMAXP6_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP6_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL6 register.
//
// *****************************************************************************
const USB_RXCSRL6_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL6_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL6_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL6_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL6_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL6_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL6_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL6_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH6 register.
//
// *****************************************************************************
const USB_RXCSRH6_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH6_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH6_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH6_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH6_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH6_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT6 register.
//
// *****************************************************************************
const USB_RXCOUNT6_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT6_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXMAXP7 register.
//
// *****************************************************************************
const USB_TXMAXP7_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_TXMAXP7_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRL7 register.
//
// *****************************************************************************
const USB_TXCSRL7_CLRDT: u32 = 0x00000040; // Clear Data Toggle
const USB_TXCSRL7_STALLED: u32 = 0x00000020; // Endpoint Stalled
const USB_TXCSRL7_STALL: u32 = 0x00000010; // Send STALL
const USB_TXCSRL7_FLUSH: u32 = 0x00000008; // Flush FIFO
const USB_TXCSRL7_UNDRN: u32 = 0x00000004; // Underrun
const USB_TXCSRL7_FIFONE: u32 = 0x00000002; // FIFO Not Empty
const USB_TXCSRL7_TXRDY: u32 = 0x00000001; // Transmit Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXCSRH7 register.
//
// *****************************************************************************
const USB_TXCSRH7_AUTOSET: u32 = 0x00000080; // Auto Set
const USB_TXCSRH7_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_TXCSRH7_MODE: u32 = 0x00000020; // Mode
const USB_TXCSRH7_DMAEN: u32 = 0x00000010; // DMA Request Enable
const USB_TXCSRH7_FDT: u32 = 0x00000008; // Force Data Toggle
const USB_TXCSRH7_DMAMOD: u32 = 0x00000004; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXMAXP7 register.
//
// *****************************************************************************
const USB_RXMAXP7_MAXLOAD_M: u32 = 0x000007FF; // Maximum Payload
const USB_RXMAXP7_MAXLOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRL7 register.
//
// *****************************************************************************
const USB_RXCSRL7_CLRDT: u32 = 0x00000080; // Clear Data Toggle
const USB_RXCSRL7_STALLED: u32 = 0x00000040; // Endpoint Stalled
const USB_RXCSRL7_STALL: u32 = 0x00000020; // Send STALL
const USB_RXCSRL7_FLUSH: u32 = 0x00000010; // Flush FIFO
const USB_RXCSRL7_DATAERR: u32 = 0x00000008; // Data Error
const USB_RXCSRL7_OVER: u32 = 0x00000004; // Overrun
const USB_RXCSRL7_FULL: u32 = 0x00000002; // FIFO Full
const USB_RXCSRL7_RXRDY: u32 = 0x00000001; // Receive Packet Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCSRH7 register.
//
// *****************************************************************************
const USB_RXCSRH7_AUTOCL: u32 = 0x00000080; // Auto Clear
const USB_RXCSRH7_ISO: u32 = 0x00000040; // Isochronous Transfers
const USB_RXCSRH7_DMAEN: u32 = 0x00000020; // DMA Request Enable
const USB_RXCSRH7_PIDERR: u32 = 0x00000010; // PID Error
const USB_RXCSRH7_DISNYET: u32 = 0x00000010; // Disable NYET
const USB_RXCSRH7_DMAMOD: u32 = 0x00000008; // DMA Request Mode

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXCOUNT7 register.
//
// *****************************************************************************
const USB_RXCOUNT7_COUNT_M: u32 = 0x00001FFF; // Receive Packet Count
const USB_RXCOUNT7_COUNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_RXDPKTBUFDIS
// register.
//
// *****************************************************************************
const USB_RXDPKTBUFDIS_EP7: u32 = 0x00000080; // EP7 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP6: u32 = 0x00000040; // EP6 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP5: u32 = 0x00000020; // EP5 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP4: u32 = 0x00000010; // EP4 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP3: u32 = 0x00000008; // EP3 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP2: u32 = 0x00000004; // EP2 RX Double-Packet Buffer Disable
const USB_RXDPKTBUFDIS_EP1: u32 = 0x00000002; // EP1 RX Double-Packet Buffer Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_TXDPKTBUFDIS
// register.
//
// *****************************************************************************
const USB_TXDPKTBUFDIS_EP7: u32 = 0x00000080; // EP7 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP6: u32 = 0x00000040; // EP6 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP5: u32 = 0x00000020; // EP5 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP4: u32 = 0x00000010; // EP4 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP3: u32 = 0x00000008; // EP3 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP2: u32 = 0x00000004; // EP2 TX Double-Packet Buffer Disable
const USB_TXDPKTBUFDIS_EP1: u32 = 0x00000002; // EP1 TX Double-Packet Buffer Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRRIS register.
//
// *****************************************************************************
const USB_DRRIS_RESUME: u32 = 0x00000001; // RESUME Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRIM register.
//
// *****************************************************************************
const USB_DRIM_RESUME: u32 = 0x00000001; // RESUME Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DRISC register.
//
// *****************************************************************************
const USB_DRISC_RESUME: u32 = 0x00000001; // RESUME Interrupt Status and Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_DMASEL register.
//
// *****************************************************************************
const USB_DMASEL_DMACTX_M: u32 = 0x00F00000; // DMA C TX Select
const USB_DMASEL_DMACRX_M: u32 = 0x000F0000; // DMA C RX Select
const USB_DMASEL_DMABTX_M: u32 = 0x0000F000; // DMA B TX Select
const USB_DMASEL_DMABRX_M: u32 = 0x00000F00; // DMA B RX Select
const USB_DMASEL_DMAATX_M: u32 = 0x000000F0; // DMA A TX Select
const USB_DMASEL_DMAARX_M: u32 = 0x0000000F; // DMA A RX Select
const USB_DMASEL_DMACTX_S: u32 = 20;
const USB_DMASEL_DMACRX_S: u32 = 16;
const USB_DMASEL_DMABTX_S: u32 = 12;
const USB_DMASEL_DMABRX_S: u32 = 8;
const USB_DMASEL_DMAATX_S: u32 = 4;
const USB_DMASEL_DMAARX_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the USB_O_PP register.
//
// *****************************************************************************
const USB_PP_ECNT_M: u32 = 0x0000FF00; // Endpoint Count
const USB_PP_USB_M: u32 = 0x000000C0; // USB Capability
const USB_PP_USB_DEVICE: u32 = 0x00000040; // DEVICE
const USB_PP_USB_HOSTDEVICE: u32 = 0x00000080; // HOST
const USB_PP_USB_OTG: u32 = 0x000000C0; // OTG
const USB_PP_PHY: u32 = 0x00000010; // PHY Present
const USB_PP_TYPE_M: u32 = 0x0000000F; // Controller Type
const USB_PP_TYPE_0: u32 = 0x00000000; // The first-generation USB controller
const USB_PP_ECNT_S: u32 = 8;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EESIZE register.
//
// *****************************************************************************
const EEPROM_EESIZE_BLKCNT_M: u32 = 0x07FF0000; // Number of 16-Word Blocks
const EEPROM_EESIZE_WORDCNT_M: u32 = 0x0000FFFF; // Number of 32-Bit Words
const EEPROM_EESIZE_BLKCNT_S: u32 = 16;
const EEPROM_EESIZE_WORDCNT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEBLOCK register.
//
// *****************************************************************************
const EEPROM_EEBLOCK_BLOCK_M: u32 = 0x0000FFFF; // Current Block
const EEPROM_EEBLOCK_BLOCK_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEOFFSET
// register.
//
// *****************************************************************************
const EEPROM_EEOFFSET_OFFSET_M: u32 = 0x0000000F; // Current Address Offset
const EEPROM_EEOFFSET_OFFSET_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EERDWR register.
//
// *****************************************************************************
const EEPROM_EERDWR_VALUE_M: u32 = 0xFFFFFFFF; // EEPROM Read or Write Data
const EEPROM_EERDWR_VALUE_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EERDWRINC
// register.
//
// *****************************************************************************
const EEPROM_EERDWRINC_VALUE_M: u32 = 0xFFFFFFFF; // EEPROM Read or Write Data with Increment
const EEPROM_EERDWRINC_VALUE_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEDONE register.
//
// *****************************************************************************
const EEPROM_EEDONE_INVPL: u32 = 0x00000100; // Invalid Program Voltage Level
const EEPROM_EEDONE_WRBUSY: u32 = 0x00000020; // Write Busy
const EEPROM_EEDONE_NOPERM: u32 = 0x00000010; // Write Without Permission
const EEPROM_EEDONE_WKCOPY: u32 = 0x00000008; // Working on a Copy
const EEPROM_EEDONE_WKERASE: u32 = 0x00000004; // Working on an Erase
const EEPROM_EEDONE_WORKING: u32 = 0x00000001; // EEPROM Working

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EESUPP register.
//
// *****************************************************************************
const EEPROM_EESUPP_PRETRY: u32 = 0x00000008; // Programming Must Be Retried
const EEPROM_EESUPP_ERETRY: u32 = 0x00000004; // Erase Must Be Retried
const EEPROM_EESUPP_EREQ: u32 = 0x00000002; // Erase Required
const EEPROM_EESUPP_START: u32 = 0x00000001; // Start Erase

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEUNLOCK
// register.
//
// *****************************************************************************
const EEPROM_EEUNLOCK_UNLOCK_M: u32 = 0xFFFFFFFF; // EEPROM Unlock

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEPROT register.
//
// *****************************************************************************
const EEPROM_EEPROT_ACC: u32 = 0x00000008; // Access Control
const EEPROM_EEPROT_PROT_M: u32 = 0x00000007; // Protection Control
const EEPROM_EEPROT_PROT_RWNPW: u32 = 0x00000000; // This setting is the default. If there is no password, the block is not protected and is readable and writable
const EEPROM_EEPROT_PROT_RWPW: u32 = 0x00000001; // If there is a password, the block is readable or writable only when unlocked
const EEPROM_EEPROT_PROT_RONPW: u32 = 0x00000002; // If there is no password, the block is readable, not writable

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEPASS0 register.
//
// *****************************************************************************
const EEPROM_EEPASS0_PASS_M: u32 = 0xFFFFFFFF; // Password
const EEPROM_EEPASS0_PASS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEPASS1 register.
//
// *****************************************************************************
const EEPROM_EEPASS1_PASS_M: u32 = 0xFFFFFFFF; // Password
const EEPROM_EEPASS1_PASS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEPASS2 register.
//
// *****************************************************************************
const EEPROM_EEPASS2_PASS_M: u32 = 0xFFFFFFFF; // Password
const EEPROM_EEPASS2_PASS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEINT register.
//
// *****************************************************************************
const EEPROM_EEINT_INT: u32 = 0x00000001; // Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEHIDE register.
//
// *****************************************************************************
const EEPROM_EEHIDE_HN_M: u32 = 0xFFFFFFFE; // Hide Block

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_EEDBGME register.
//
// *****************************************************************************
const EEPROM_EEDBGME_KEY_M: u32 = 0xFFFF0000; // Erase Key
const EEPROM_EEDBGME_ME: u32 = 0x00000001; // Mass Erase
const EEPROM_EEDBGME_KEY_S: u32 = 16;

// *****************************************************************************
//
// The following are defines for the bit fields in the EEPROM_PP register.
//
// *****************************************************************************
const EEPROM_PP_SIZE_M: u32 = 0x0000001F; // EEPROM Size
const EEPROM_PP_SIZE_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSEXC_RIS register.
//
// *****************************************************************************
const SYSEXC_RIS_FPIXCRIS: u32 = 0x00000020; // Floating-Point Inexact Exception Raw Interrupt Status
const SYSEXC_RIS_FPOFCRIS: u32 = 0x00000010; // Floating-Point Overflow Exception Raw Interrupt Status
const SYSEXC_RIS_FPUFCRIS: u32 = 0x00000008; // Floating-Point Underflow Exception Raw Interrupt Status
const SYSEXC_RIS_FPIOCRIS: u32 = 0x00000004; // Floating-Point Invalid Operation Raw Interrupt Status
const SYSEXC_RIS_FPDZCRIS: u32 = 0x00000002; // Floating-Point Divide By 0 Exception Raw Interrupt Status
const SYSEXC_RIS_FPIDCRIS: u32 = 0x00000001; // Floating-Point Input Denormal Exception Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSEXC_IM register.
//
// *****************************************************************************
const SYSEXC_IM_FPIXCIM: u32 = 0x00000020; // Floating-Point Inexact Exception Interrupt Mask
const SYSEXC_IM_FPOFCIM: u32 = 0x00000010; // Floating-Point Overflow Exception Interrupt Mask
const SYSEXC_IM_FPUFCIM: u32 = 0x00000008; // Floating-Point Underflow Exception Interrupt Mask
const SYSEXC_IM_FPIOCIM: u32 = 0x00000004; // Floating-Point Invalid Operation Interrupt Mask
const SYSEXC_IM_FPDZCIM: u32 = 0x00000002; // Floating-Point Divide By 0 Exception Interrupt Mask
const SYSEXC_IM_FPIDCIM: u32 = 0x00000001; // Floating-Point Input Denormal Exception Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSEXC_MIS register.
//
// *****************************************************************************
const SYSEXC_MIS_FPIXCMIS: u32 = 0x00000020; // Floating-Point Inexact Exception Masked Interrupt Status
const SYSEXC_MIS_FPOFCMIS: u32 = 0x00000010; // Floating-Point Overflow Exception Masked Interrupt Status
const SYSEXC_MIS_FPUFCMIS: u32 = 0x00000008; // Floating-Point Underflow Exception Masked Interrupt Status
const SYSEXC_MIS_FPIOCMIS: u32 = 0x00000004; // Floating-Point Invalid Operation Masked Interrupt Status
const SYSEXC_MIS_FPDZCMIS: u32 = 0x00000002; // Floating-Point Divide By 0 Exception Masked Interrupt Status
const SYSEXC_MIS_FPIDCMIS: u32 = 0x00000001; // Floating-Point Input Denormal Exception Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSEXC_IC register.
//
// *****************************************************************************
const SYSEXC_IC_FPIXCIC: u32 = 0x00000020; // Floating-Point Inexact Exception Interrupt Clear
const SYSEXC_IC_FPOFCIC: u32 = 0x00000010; // Floating-Point Overflow Exception Interrupt Clear
const SYSEXC_IC_FPUFCIC: u32 = 0x00000008; // Floating-Point Underflow Exception Interrupt Clear
const SYSEXC_IC_FPIOCIC: u32 = 0x00000004; // Floating-Point Invalid Operation Interrupt Clear
const SYSEXC_IC_FPDZCIC: u32 = 0x00000002; // Floating-Point Divide By 0 Exception Interrupt Clear
const SYSEXC_IC_FPIDCIC: u32 = 0x00000001; // Floating-Point Input Denormal Exception Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RTCC register.
//
// *****************************************************************************
const HIB_RTCC_M: u32 = 0xFFFFFFFF; // RTC Counter
const HIB_RTCC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RTCM0 register.
//
// *****************************************************************************
const HIB_RTCM0_M: u32 = 0xFFFFFFFF; // RTC Match 0
const HIB_RTCM0_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RTCLD register.
//
// *****************************************************************************
const HIB_RTCLD_M: u32 = 0xFFFFFFFF; // RTC Load
const HIB_RTCLD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_CTL register.
//
// *****************************************************************************
const HIB_CTL_WRC: u32 = 0x80000000; // Write Complete/Capable
const HIB_CTL_OSCDRV: u32 = 0x00020000; // Oscillator Drive Capability
const HIB_CTL_OSCBYP: u32 = 0x00010000; // Oscillator Bypass
const HIB_CTL_VBATSEL_M: u32 = 0x00006000; // Select for Low-Battery Comparator
const HIB_CTL_VBATSEL_1_9V: u32 = 0x00000000; // 1.9 Volts
const HIB_CTL_VBATSEL_2_1V: u32 = 0x00002000; // 2.1 Volts (default)
const HIB_CTL_VBATSEL_2_3V: u32 = 0x00004000; // 2.3 Volts
const HIB_CTL_VBATSEL_2_5V: u32 = 0x00006000; // 2.5 Volts
const HIB_CTL_BATCHK: u32 = 0x00000400; // Check Battery Status
const HIB_CTL_BATWKEN: u32 = 0x00000200; // Wake on Low Battery
const HIB_CTL_VDD3ON: u32 = 0x00000100; // VDD Powered
const HIB_CTL_VABORT: u32 = 0x00000080; // Power Cut Abort Enable
const HIB_CTL_CLK32EN: u32 = 0x00000040; // Clocking Enable
const HIB_CTL_LOWBATEN: u32 = 0x00000020; // Low Battery Monitoring Enable
const HIB_CTL_PINWEN: u32 = 0x00000010; // External WAKE Pin Enable
const HIB_CTL_RTCWEN: u32 = 0x00000008; // RTC Wake-up Enable
const HIB_CTL_HIBREQ: u32 = 0x00000002; // Hibernation Request
const HIB_CTL_RTCEN: u32 = 0x00000001; // RTC Timer Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_IM register.
//
// *****************************************************************************
const HIB_IM_WC: u32 = 0x00000010; // External Write Complete/Capable Interrupt Mask
const HIB_IM_EXTW: u32 = 0x00000008; // External Wake-Up Interrupt Mask
const HIB_IM_LOWBAT: u32 = 0x00000004; // Low Battery Voltage Interrupt Mask
const HIB_IM_RTCALT0: u32 = 0x00000001; // RTC Alert 0 Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RIS register.
//
// *****************************************************************************
const HIB_RIS_WC: u32 = 0x00000010; // Write Complete/Capable Raw Interrupt Status
const HIB_RIS_EXTW: u32 = 0x00000008; // External Wake-Up Raw Interrupt Status
const HIB_RIS_LOWBAT: u32 = 0x00000004; // Low Battery Voltage Raw Interrupt Status
const HIB_RIS_RTCALT0: u32 = 0x00000001; // RTC Alert 0 Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_MIS register.
//
// *****************************************************************************
const HIB_MIS_WC: u32 = 0x00000010; // Write Complete/Capable Masked Interrupt Status
const HIB_MIS_EXTW: u32 = 0x00000008; // External Wake-Up Masked Interrupt Status
const HIB_MIS_LOWBAT: u32 = 0x00000004; // Low Battery Voltage Masked Interrupt Status
const HIB_MIS_RTCALT0: u32 = 0x00000001; // RTC Alert 0 Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_IC register.
//
// *****************************************************************************
const HIB_IC_WC: u32 = 0x00000010; // Write Complete/Capable Masked Interrupt Clear
const HIB_IC_EXTW: u32 = 0x00000008; // External Wake-Up Masked Interrupt Clear
const HIB_IC_LOWBAT: u32 = 0x00000004; // Low Battery Voltage Masked Interrupt Clear
const HIB_IC_RTCALT0: u32 = 0x00000001; // RTC Alert0 Masked Interrupt Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RTCT register.
//
// *****************************************************************************
const HIB_RTCT_TRIM_M: u32 = 0x0000FFFF; // RTC Trim Value
const HIB_RTCT_TRIM_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_RTCSS register.
//
// *****************************************************************************
const HIB_RTCSS_RTCSSM_M: u32 = 0x7FFF0000; // RTC Sub Seconds Match
const HIB_RTCSS_RTCSSC_M: u32 = 0x00007FFF; // RTC Sub Seconds Count
const HIB_RTCSS_RTCSSM_S: u32 = 16;
const HIB_RTCSS_RTCSSC_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the HIB_DATA register.
//
// *****************************************************************************
const HIB_DATA_RTD_M: u32 = 0xFFFFFFFF; // Hibernation Module NV Data
const HIB_DATA_RTD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FMA register.
//
// *****************************************************************************
const FLASH_FMA_OFFSET_M: u32 = 0x0003FFFF; // Address Offset
const FLASH_FMA_OFFSET_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FMD register.
//
// *****************************************************************************
const FLASH_FMD_DATA_M: u32 = 0xFFFFFFFF; // Data Value
const FLASH_FMD_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FMC register.
//
// *****************************************************************************
const FLASH_FMC_WRKEY: u32 = 0xA4420000; // FLASH write key
const FLASH_FMC_COMT: u32 = 0x00000008; // Commit Register Value
const FLASH_FMC_MERASE: u32 = 0x00000004; // Mass Erase Flash Memory
const FLASH_FMC_ERASE: u32 = 0x00000002; // Erase a Page of Flash Memory
const FLASH_FMC_WRITE: u32 = 0x00000001; // Write a Word into Flash Memory

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FCRIS register.
//
// *****************************************************************************
const FLASH_FCRIS_PROGRIS: u32 = 0x00002000; // PROGVER Raw Interrupt Status
const FLASH_FCRIS_ERRIS: u32 = 0x00000800; // ERVER Raw Interrupt Status
const FLASH_FCRIS_INVDRIS: u32 = 0x00000400; // Invalid Data Raw Interrupt Status
const FLASH_FCRIS_VOLTRIS: u32 = 0x00000200; // VOLTSTAT Raw Interrupt Status
const FLASH_FCRIS_ERIS: u32 = 0x00000004; // EEPROM Raw Interrupt Status
const FLASH_FCRIS_PRIS: u32 = 0x00000002; // Programming Raw Interrupt Status
const FLASH_FCRIS_ARIS: u32 = 0x00000001; // Access Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FCIM register.
//
// *****************************************************************************
const FLASH_FCIM_PROGMASK: u32 = 0x00002000; // PROGVER Interrupt Mask
const FLASH_FCIM_ERMASK: u32 = 0x00000800; // ERVER Interrupt Mask
const FLASH_FCIM_INVDMASK: u32 = 0x00000400; // Invalid Data Interrupt Mask
const FLASH_FCIM_VOLTMASK: u32 = 0x00000200; // VOLT Interrupt Mask
const FLASH_FCIM_EMASK: u32 = 0x00000004; // EEPROM Interrupt Mask
const FLASH_FCIM_PMASK: u32 = 0x00000002; // Programming Interrupt Mask
const FLASH_FCIM_AMASK: u32 = 0x00000001; // Access Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FCMISC register.
//
// *****************************************************************************
const FLASH_FCMISC_PROGMISC: u32 = 0x00002000; // PROGVER Masked Interrupt Status and Clear
const FLASH_FCMISC_ERMISC: u32 = 0x00000800; // ERVER Masked Interrupt Status and Clear
const FLASH_FCMISC_INVDMISC: u32 = 0x00000400; // Invalid Data Masked Interrupt Status and Clear
const FLASH_FCMISC_VOLTMISC: u32 = 0x00000200; // VOLT Masked Interrupt Status and Clear
const FLASH_FCMISC_EMISC: u32 = 0x00000004; // EEPROM Masked Interrupt Status and Clear
const FLASH_FCMISC_PMISC: u32 = 0x00000002; // Programming Masked Interrupt Status and Clear
const FLASH_FCMISC_AMISC: u32 = 0x00000001; // Access Masked Interrupt Status and Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FMC2 register.
//
// *****************************************************************************
const FLASH_FMC2_WRKEY: u32 = 0xA4420000; // FLASH write key
const FLASH_FMC2_WRBUF: u32 = 0x00000001; // Buffered Flash Memory Write

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FWBVAL register.
//
// *****************************************************************************
const FLASH_FWBVAL_FWB_M: u32 = 0xFFFFFFFF; // Flash Memory Write Buffer

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FWBN register.
//
// *****************************************************************************
const FLASH_FWBN_DATA_M: u32 = 0xFFFFFFFF; // Data

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_FSIZE register.
//
// *****************************************************************************
const FLASH_FSIZE_SIZE_M: u32 = 0x0000FFFF; // Flash Size
const FLASH_FSIZE_SIZE_8KB: u32 = 0x00000003; // 8 KB of Flash
const FLASH_FSIZE_SIZE_16KB: u32 = 0x00000007; // 16 KB of Flash
const FLASH_FSIZE_SIZE_32KB: u32 = 0x0000000F; // 32 KB of Flash
const FLASH_FSIZE_SIZE_64KB: u32 = 0x0000001F; // 64 KB of Flash
const FLASH_FSIZE_SIZE_96KB: u32 = 0x0000002F; // 96 KB of Flash
const FLASH_FSIZE_SIZE_128KB: u32 = 0x0000003F; // 128 KB of Flash
const FLASH_FSIZE_SIZE_192KB: u32 = 0x0000005F; // 192 KB of Flash
const FLASH_FSIZE_SIZE_256KB: u32 = 0x0000007F; // 256 KB of Flash

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_SSIZE register.
//
// *****************************************************************************
const FLASH_SSIZE_SIZE_M: u32 = 0x0000FFFF; // SRAM Size
const FLASH_SSIZE_SIZE_2KB: u32 = 0x00000007; // 2 KB of SRAM
const FLASH_SSIZE_SIZE_4KB: u32 = 0x0000000F; // 4 KB of SRAM
const FLASH_SSIZE_SIZE_6KB: u32 = 0x00000017; // 6 KB of SRAM
const FLASH_SSIZE_SIZE_8KB: u32 = 0x0000001F; // 8 KB of SRAM
const FLASH_SSIZE_SIZE_12KB: u32 = 0x0000002F; // 12 KB of SRAM
const FLASH_SSIZE_SIZE_16KB: u32 = 0x0000003F; // 16 KB of SRAM
const FLASH_SSIZE_SIZE_20KB: u32 = 0x0000004F; // 20 KB of SRAM
const FLASH_SSIZE_SIZE_24KB: u32 = 0x0000005F; // 24 KB of SRAM
const FLASH_SSIZE_SIZE_32KB: u32 = 0x0000007F; // 32 KB of SRAM

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_ROMSWMAP register.
//
// *****************************************************************************
const FLASH_ROMSWMAP_SAFERTOS: u32 = 0x00000001; // SafeRTOS Present

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_RMCTL register.
//
// *****************************************************************************
const FLASH_RMCTL_BA: u32 = 0x00000001; // Boot Alias

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_BOOTCFG register.
//
// *****************************************************************************
const FLASH_BOOTCFG_NW: u32 = 0x80000000; // Not Written
const FLASH_BOOTCFG_PORT_M: u32 = 0x0000E000; // Boot GPIO Port
const FLASH_BOOTCFG_PORT_A: u32 = 0x00000000; // Port A
const FLASH_BOOTCFG_PORT_B: u32 = 0x00002000; // Port B
const FLASH_BOOTCFG_PORT_C: u32 = 0x00004000; // Port C
const FLASH_BOOTCFG_PORT_D: u32 = 0x00006000; // Port D
const FLASH_BOOTCFG_PORT_E: u32 = 0x00008000; // Port E
const FLASH_BOOTCFG_PORT_F: u32 = 0x0000A000; // Port F
const FLASH_BOOTCFG_PORT_G: u32 = 0x0000C000; // Port G
const FLASH_BOOTCFG_PORT_H: u32 = 0x0000E000; // Port H
const FLASH_BOOTCFG_PIN_M: u32 = 0x00001C00; // Boot GPIO Pin
const FLASH_BOOTCFG_PIN_0: u32 = 0x00000000; // Pin 0
const FLASH_BOOTCFG_PIN_1: u32 = 0x00000400; // Pin 1
const FLASH_BOOTCFG_PIN_2: u32 = 0x00000800; // Pin 2
const FLASH_BOOTCFG_PIN_3: u32 = 0x00000C00; // Pin 3
const FLASH_BOOTCFG_PIN_4: u32 = 0x00001000; // Pin 4
const FLASH_BOOTCFG_PIN_5: u32 = 0x00001400; // Pin 5
const FLASH_BOOTCFG_PIN_6: u32 = 0x00001800; // Pin 6
const FLASH_BOOTCFG_PIN_7: u32 = 0x00001C00; // Pin 7
const FLASH_BOOTCFG_POL: u32 = 0x00000200; // Boot GPIO Polarity
const FLASH_BOOTCFG_EN: u32 = 0x00000100; // Boot GPIO Enable
const FLASH_BOOTCFG_DBG1: u32 = 0x00000002; // Debug Control 1
const FLASH_BOOTCFG_DBG0: u32 = 0x00000001; // Debug Control 0

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_USERREG0 register.
//
// *****************************************************************************
const FLASH_USERREG0_DATA_M: u32 = 0xFFFFFFFF; // User Data
const FLASH_USERREG0_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_USERREG1 register.
//
// *****************************************************************************
const FLASH_USERREG1_DATA_M: u32 = 0xFFFFFFFF; // User Data
const FLASH_USERREG1_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_USERREG2 register.
//
// *****************************************************************************
const FLASH_USERREG2_DATA_M: u32 = 0xFFFFFFFF; // User Data
const FLASH_USERREG2_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the FLASH_USERREG3 register.
//
// *****************************************************************************
const FLASH_USERREG3_DATA_M: u32 = 0xFFFFFFFF; // User Data
const FLASH_USERREG3_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DID0 register.
//
// *****************************************************************************
const SYSCTL_DID0_VER_M: u32 = 0x70000000; // DID0 Version
const SYSCTL_DID0_VER_1: u32 = 0x10000000; // Second version of the DID0 register format
const SYSCTL_DID0_CLASS_M: u32 = 0x00FF0000; // Device Class
const SYSCTL_DID0_CLASS_BLIZZARD: u32 = 0x00050000; // Stellaris(R) Blizzard-class microcontrollers
const SYSCTL_DID0_MAJ_M: u32 = 0x0000FF00; // Major Revision
const SYSCTL_DID0_MAJ_REVA: u32 = 0x00000000; // Revision A (initial device)
const SYSCTL_DID0_MAJ_REVB: u32 = 0x00000100; // Revision B (first base layer revision)
const SYSCTL_DID0_MAJ_REVC: u32 = 0x00000200; // Revision C (second base layer revision)
const SYSCTL_DID0_MIN_M: u32 = 0x000000FF; // Minor Revision
const SYSCTL_DID0_MIN_0: u32 = 0x00000000; // Initial device, or a major revision update
const SYSCTL_DID0_MIN_1: u32 = 0x00000001; // First metal layer change
const SYSCTL_DID0_MIN_2: u32 = 0x00000002; // Second metal layer change

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DID1 register.
//
// *****************************************************************************
const SYSCTL_DID1_VER_M: u32 = 0xF0000000; // DID1 Version
const SYSCTL_DID1_VER_0: u32 = 0x00000000; // Initial DID1 register format definition, indicating a Stellaris LM3Snnn device
const SYSCTL_DID1_VER_1: u32 = 0x10000000; // Second version of the DID1 register format
const SYSCTL_DID1_FAM_M: u32 = 0x0F000000; // Family
const SYSCTL_DID1_FAM_STELLARIS: u32 = 0x00000000; // Stellaris family of microcontollers, that is, all devices with external part numbers starting with LM3S
const SYSCTL_DID1_PRTNO_M: u32 = 0x00FF0000; // Part Number
const SYSCTL_DID1_PRTNO_LM4F120H5QR: u32 = 0x00040000; // LM4F120H5QR
const SYSCTL_DID1_PINCNT_M: u32 = 0x0000E000; // Package Pin Count
const SYSCTL_DID1_PINCNT_28: u32 = 0x00000000; // 28-pin package
const SYSCTL_DID1_PINCNT_48: u32 = 0x00002000; // 48-pin package
const SYSCTL_DID1_PINCNT_100: u32 = 0x00004000; // 100-pin package
const SYSCTL_DID1_PINCNT_64: u32 = 0x00006000; // 64-pin package
const SYSCTL_DID1_PINCNT_144: u32 = 0x00008000; // 144-pin package
const SYSCTL_DID1_PINCNT_157: u32 = 0x0000A000; // 157-pin package
const SYSCTL_DID1_TEMP_M: u32 = 0x000000E0; // Temperature Range
const SYSCTL_DID1_TEMP_C: u32 = 0x00000000; // Commercial temperature range (0C to 70C)
const SYSCTL_DID1_TEMP_I: u32 = 0x00000020; // Industrial temperature range (-40C to 85C)
const SYSCTL_DID1_TEMP_E: u32 = 0x00000040; // Extended temperature range (-40C to 105C)
const SYSCTL_DID1_PKG_M: u32 = 0x00000018; // Package Type
const SYSCTL_DID1_PKG_SOIC: u32 = 0x00000000; // SOIC package
const SYSCTL_DID1_PKG_QFP: u32 = 0x00000008; // LQFP package
const SYSCTL_DID1_PKG_BGA: u32 = 0x00000010; // BGA package
const SYSCTL_DID1_ROHS: u32 = 0x00000004; // RoHS-Compliance
const SYSCTL_DID1_QUAL_M: u32 = 0x00000003; // Qualification Status
const SYSCTL_DID1_QUAL_ES: u32 = 0x00000000; // Engineering Sample (unqualified)
const SYSCTL_DID1_QUAL_PP: u32 = 0x00000001; // Pilot Production (unqualified)
const SYSCTL_DID1_QUAL_FQ: u32 = 0x00000002; // Fully Qualified

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC0 register.
//
// *****************************************************************************
const SYSCTL_DC0_SRAMSZ_M: u32 = 0xFFFF0000; // SRAM Size
const SYSCTL_DC0_SRAMSZ_2KB: u32 = 0x00070000; // 2 KB of SRAM
const SYSCTL_DC0_SRAMSZ_4KB: u32 = 0x000F0000; // 4 KB of SRAM
const SYSCTL_DC0_SRAMSZ_6KB: u32 = 0x00170000; // 6 KB of SRAM
const SYSCTL_DC0_SRAMSZ_8KB: u32 = 0x001F0000; // 8 KB of SRAM
const SYSCTL_DC0_SRAMSZ_12KB: u32 = 0x002F0000; // 12 KB of SRAM
const SYSCTL_DC0_SRAMSZ_16KB: u32 = 0x003F0000; // 16 KB of SRAM
const SYSCTL_DC0_SRAMSZ_20KB: u32 = 0x004F0000; // 20 KB of SRAM
const SYSCTL_DC0_SRAMSZ_24KB: u32 = 0x005F0000; // 24 KB of SRAM
const SYSCTL_DC0_SRAMSZ_32KB: u32 = 0x007F0000; // 32 KB of SRAM
const SYSCTL_DC0_FLASHSZ_M: u32 = 0x0000FFFF; // Flash Size
const SYSCTL_DC0_FLASHSZ_8KB: u32 = 0x00000003; // 8 KB of Flash
const SYSCTL_DC0_FLASHSZ_16KB: u32 = 0x00000007; // 16 KB of Flash
const SYSCTL_DC0_FLASHSZ_32KB: u32 = 0x0000000F; // 32 KB of Flash
const SYSCTL_DC0_FLASHSZ_64KB: u32 = 0x0000001F; // 64 KB of Flash
const SYSCTL_DC0_FLASHSZ_96KB: u32 = 0x0000002F; // 96 KB of Flash
const SYSCTL_DC0_FLASHSZ_128K: u32 = 0x0000003F; // 128 KB of Flash
const SYSCTL_DC0_FLASHSZ_192K: u32 = 0x0000005F; // 192 KB of Flash
const SYSCTL_DC0_FLASHSZ_256K: u32 = 0x0000007F; // 256 KB of Flash
const SYSCTL_DC0_SRAMSZ_S: u32 = 16; // SRAM size shift
const SYSCTL_DC0_FLASHSZ_S: u32 = 0; // Flash size shift

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC1 register.
//
// *****************************************************************************
const SYSCTL_DC1_WDT1: u32 = 0x10000000; // Watchdog Timer1 Present
const SYSCTL_DC1_CAN1: u32 = 0x02000000; // CAN Module 1 Present
const SYSCTL_DC1_CAN0: u32 = 0x01000000; // CAN Module 0 Present
const SYSCTL_DC1_PWM1: u32 = 0x00200000; // PWM Module 1 Present
const SYSCTL_DC1_PWM0: u32 = 0x00100000; // PWM Module 0 Present
const SYSCTL_DC1_ADC1: u32 = 0x00020000; // ADC Module 1 Present
const SYSCTL_DC1_ADC0: u32 = 0x00010000; // ADC Module 0 Present
const SYSCTL_DC1_MINSYSDIV_M: u32 = 0x0000F000; // System Clock Divider
const SYSCTL_DC1_MINSYSDIV_100: u32 = 0x00001000; // Divide VCO (400MHZ) by 5 minimum
const SYSCTL_DC1_MINSYSDIV_66: u32 = 0x00002000; // Divide VCO (400MHZ) by 2*2 + 2 = 6 minimum
const SYSCTL_DC1_MINSYSDIV_50: u32 = 0x00003000; // Specifies a 50-MHz CPU clock with a PLL divider of 4
const SYSCTL_DC1_MINSYSDIV_40: u32 = 0x00004000; // Specifies a 40-MHz CPU clock with a PLL divider of 5
const SYSCTL_DC1_MINSYSDIV_25: u32 = 0x00007000; // Specifies a 25-MHz clock with a PLL divider of 8
const SYSCTL_DC1_MINSYSDIV_20: u32 = 0x00009000; // Specifies a 20-MHz clock with a PLL divider of 10
const SYSCTL_DC1_ADC1SPD_M: u32 = 0x00000C00; // Max ADC1 Speed
const SYSCTL_DC1_ADC1SPD_125K: u32 = 0x00000000; // 125K samples/second
const SYSCTL_DC1_ADC1SPD_250K: u32 = 0x00000400; // 250K samples/second
const SYSCTL_DC1_ADC1SPD_500K: u32 = 0x00000800; // 500K samples/second
const SYSCTL_DC1_ADC1SPD_1M: u32 = 0x00000C00; // 1M samples/second
const SYSCTL_DC1_ADC0SPD_M: u32 = 0x00000300; // Max ADC0 Speed
const SYSCTL_DC1_ADC0SPD_125K: u32 = 0x00000000; // 125K samples/second
const SYSCTL_DC1_ADC0SPD_250K: u32 = 0x00000100; // 250K samples/second
const SYSCTL_DC1_ADC0SPD_500K: u32 = 0x00000200; // 500K samples/second
const SYSCTL_DC1_ADC0SPD_1M: u32 = 0x00000300; // 1M samples/second
const SYSCTL_DC1_MPU: u32 = 0x00000080; // MPU Present
const SYSCTL_DC1_HIB: u32 = 0x00000040; // Hibernation Module Present
const SYSCTL_DC1_TEMP: u32 = 0x00000020; // Temp Sensor Present
const SYSCTL_DC1_PLL: u32 = 0x00000010; // PLL Present
const SYSCTL_DC1_WDT0: u32 = 0x00000008; // Watchdog Timer 0 Present
const SYSCTL_DC1_SWO: u32 = 0x00000004; // SWO Trace Port Present
const SYSCTL_DC1_SWD: u32 = 0x00000002; // SWD Present
const SYSCTL_DC1_JTAG: u32 = 0x00000001; // JTAG Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC2 register.
//
// *****************************************************************************
const SYSCTL_DC2_EPI0: u32 = 0x40000000; // EPI Module 0 Present
const SYSCTL_DC2_I2S0: u32 = 0x10000000; // I2S Module 0 Present
const SYSCTL_DC2_COMP2: u32 = 0x04000000; // Analog Comparator 2 Present
const SYSCTL_DC2_COMP1: u32 = 0x02000000; // Analog Comparator 1 Present
const SYSCTL_DC2_COMP0: u32 = 0x01000000; // Analog Comparator 0 Present
const SYSCTL_DC2_TIMER3: u32 = 0x00080000; // Timer Module 3 Present
const SYSCTL_DC2_TIMER2: u32 = 0x00040000; // Timer Module 2 Present
const SYSCTL_DC2_TIMER1: u32 = 0x00020000; // Timer Module 1 Present
const SYSCTL_DC2_TIMER0: u32 = 0x00010000; // Timer Module 0 Present
const SYSCTL_DC2_I2C1HS: u32 = 0x00008000; // I2C Module 1 Speed
const SYSCTL_DC2_I2C1: u32 = 0x00004000; // I2C Module 1 Present
const SYSCTL_DC2_I2C0HS: u32 = 0x00002000; // I2C Module 0 Speed
const SYSCTL_DC2_I2C0: u32 = 0x00001000; // I2C Module 0 Present
const SYSCTL_DC2_QEI1: u32 = 0x00000200; // QEI Module 1 Present
const SYSCTL_DC2_QEI0: u32 = 0x00000100; // QEI Module 0 Present
const SYSCTL_DC2_SSI1: u32 = 0x00000020; // SSI Module 1 Present
const SYSCTL_DC2_SSI0: u32 = 0x00000010; // SSI Module 0 Present
const SYSCTL_DC2_UART2: u32 = 0x00000004; // UART Module 2 Present
const SYSCTL_DC2_UART1: u32 = 0x00000002; // UART Module 1 Present
const SYSCTL_DC2_UART0: u32 = 0x00000001; // UART Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC3 register.
//
// *****************************************************************************
const SYSCTL_DC3_32KHZ: u32 = 0x80000000; // 32KHz Input Clock Available
const SYSCTL_DC3_CCP5: u32 = 0x20000000; // CCP5 Pin Present
const SYSCTL_DC3_CCP4: u32 = 0x10000000; // CCP4 Pin Present
const SYSCTL_DC3_CCP3: u32 = 0x08000000; // CCP3 Pin Present
const SYSCTL_DC3_CCP2: u32 = 0x04000000; // CCP2 Pin Present
const SYSCTL_DC3_CCP1: u32 = 0x02000000; // CCP1 Pin Present
const SYSCTL_DC3_CCP0: u32 = 0x01000000; // CCP0 Pin Present
const SYSCTL_DC3_ADC0AIN7: u32 = 0x00800000; // ADC Module 0 AIN7 Pin Present
const SYSCTL_DC3_ADC0AIN6: u32 = 0x00400000; // ADC Module 0 AIN6 Pin Present
const SYSCTL_DC3_ADC0AIN5: u32 = 0x00200000; // ADC Module 0 AIN5 Pin Present
const SYSCTL_DC3_ADC0AIN4: u32 = 0x00100000; // ADC Module 0 AIN4 Pin Present
const SYSCTL_DC3_ADC0AIN3: u32 = 0x00080000; // ADC Module 0 AIN3 Pin Present
const SYSCTL_DC3_ADC0AIN2: u32 = 0x00040000; // ADC Module 0 AIN2 Pin Present
const SYSCTL_DC3_ADC0AIN1: u32 = 0x00020000; // ADC Module 0 AIN1 Pin Present
const SYSCTL_DC3_ADC0AIN0: u32 = 0x00010000; // ADC Module 0 AIN0 Pin Present
const SYSCTL_DC3_PWMFAULT: u32 = 0x00008000; // PWM Fault Pin Present
const SYSCTL_DC3_C2O: u32 = 0x00004000; // C2o Pin Present
const SYSCTL_DC3_C2PLUS: u32 = 0x00002000; // C2+ Pin Present
const SYSCTL_DC3_C2MINUS: u32 = 0x00001000; // C2- Pin Present
const SYSCTL_DC3_C1O: u32 = 0x00000800; // C1o Pin Present
const SYSCTL_DC3_C1PLUS: u32 = 0x00000400; // C1+ Pin Present
const SYSCTL_DC3_C1MINUS: u32 = 0x00000200; // C1- Pin Present
const SYSCTL_DC3_C0O: u32 = 0x00000100; // C0o Pin Present
const SYSCTL_DC3_C0PLUS: u32 = 0x00000080; // C0+ Pin Present
const SYSCTL_DC3_C0MINUS: u32 = 0x00000040; // C0- Pin Present
const SYSCTL_DC3_PWM5: u32 = 0x00000020; // PWM5 Pin Present
const SYSCTL_DC3_PWM4: u32 = 0x00000010; // PWM4 Pin Present
const SYSCTL_DC3_PWM3: u32 = 0x00000008; // PWM3 Pin Present
const SYSCTL_DC3_PWM2: u32 = 0x00000004; // PWM2 Pin Present
const SYSCTL_DC3_PWM1: u32 = 0x00000002; // PWM1 Pin Present
const SYSCTL_DC3_PWM0: u32 = 0x00000001; // PWM0 Pin Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC4 register.
//
// *****************************************************************************
const SYSCTL_DC4_EPHY0: u32 = 0x40000000; // Ethernet PHY Layer 0 Present
const SYSCTL_DC4_EMAC0: u32 = 0x10000000; // Ethernet MAC Layer 0 Present
const SYSCTL_DC4_E1588: u32 = 0x01000000; // 1588 Capable
const SYSCTL_DC4_PICAL: u32 = 0x00040000; // PIOSC Calibrate
const SYSCTL_DC4_CCP7: u32 = 0x00008000; // CCP7 Pin Present
const SYSCTL_DC4_CCP6: u32 = 0x00004000; // CCP6 Pin Present
const SYSCTL_DC4_UDMA: u32 = 0x00002000; // Micro-DMA Module Present
const SYSCTL_DC4_ROM: u32 = 0x00001000; // Internal Code ROM Present
const SYSCTL_DC4_GPIOJ: u32 = 0x00000100; // GPIO Port J Present
const SYSCTL_DC4_GPIOH: u32 = 0x00000080; // GPIO Port H Present
const SYSCTL_DC4_GPIOG: u32 = 0x00000040; // GPIO Port G Present
const SYSCTL_DC4_GPIOF: u32 = 0x00000020; // GPIO Port F Present
const SYSCTL_DC4_GPIOE: u32 = 0x00000010; // GPIO Port E Present
const SYSCTL_DC4_GPIOD: u32 = 0x00000008; // GPIO Port D Present
const SYSCTL_DC4_GPIOC: u32 = 0x00000004; // GPIO Port C Present
const SYSCTL_DC4_GPIOB: u32 = 0x00000002; // GPIO Port B Present
const SYSCTL_DC4_GPIOA: u32 = 0x00000001; // GPIO Port A Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC5 register.
//
// *****************************************************************************
const SYSCTL_DC5_PWMFAULT3: u32 = 0x08000000; // PWM Fault 3 Pin Present
const SYSCTL_DC5_PWMFAULT2: u32 = 0x04000000; // PWM Fault 2 Pin Present
const SYSCTL_DC5_PWMFAULT1: u32 = 0x02000000; // PWM Fault 1 Pin Present
const SYSCTL_DC5_PWMFAULT0: u32 = 0x01000000; // PWM Fault 0 Pin Present
const SYSCTL_DC5_PWMEFLT: u32 = 0x00200000; // PWM Extended Fault Active
const SYSCTL_DC5_PWMESYNC: u32 = 0x00100000; // PWM Extended SYNC Active
const SYSCTL_DC5_PWM7: u32 = 0x00000080; // PWM7 Pin Present
const SYSCTL_DC5_PWM6: u32 = 0x00000040; // PWM6 Pin Present
const SYSCTL_DC5_PWM5: u32 = 0x00000020; // PWM5 Pin Present
const SYSCTL_DC5_PWM4: u32 = 0x00000010; // PWM4 Pin Present
const SYSCTL_DC5_PWM3: u32 = 0x00000008; // PWM3 Pin Present
const SYSCTL_DC5_PWM2: u32 = 0x00000004; // PWM2 Pin Present
const SYSCTL_DC5_PWM1: u32 = 0x00000002; // PWM1 Pin Present
const SYSCTL_DC5_PWM0: u32 = 0x00000001; // PWM0 Pin Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC6 register.
//
// *****************************************************************************
const SYSCTL_DC6_USB0PHY: u32 = 0x00000010; // USB Module 0 PHY Present
const SYSCTL_DC6_USB0_M: u32 = 0x00000003; // USB Module 0 Present
const SYSCTL_DC6_USB0_DEV: u32 = 0x00000001; // USB0 is Device Only
const SYSCTL_DC6_USB0_HOSTDEV: u32 = 0x00000002; // USB is Device or Host
const SYSCTL_DC6_USB0_OTG: u32 = 0x00000003; // USB0 is OTG

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC7 register.
//
// *****************************************************************************
const SYSCTL_DC7_DMACH30: u32 = 0x40000000; // SW
const SYSCTL_DC7_DMACH29: u32 = 0x20000000; // I2S0_TX / CAN1_TX
const SYSCTL_DC7_DMACH28: u32 = 0x10000000; // I2S0_RX / CAN1_RX
const SYSCTL_DC7_DMACH27: u32 = 0x08000000; // CAN1_TX / ADC1_SS3
const SYSCTL_DC7_DMACH26: u32 = 0x04000000; // CAN1_RX / ADC1_SS2
const SYSCTL_DC7_DMACH25: u32 = 0x02000000; // SSI1_TX / ADC1_SS1
const SYSCTL_DC7_DMACH24: u32 = 0x01000000; // SSI1_RX / ADC1_SS0
const SYSCTL_DC7_DMACH23: u32 = 0x00800000; // UART1_TX / CAN2_TX
const SYSCTL_DC7_DMACH22: u32 = 0x00400000; // UART1_RX / CAN2_RX
const SYSCTL_DC7_DMACH21: u32 = 0x00200000; // Timer1B / EPI0_WFIFO
const SYSCTL_DC7_DMACH20: u32 = 0x00100000; // Timer1A / EPI0_NBRFIFO
const SYSCTL_DC7_DMACH19: u32 = 0x00080000; // Timer0B / Timer1B
const SYSCTL_DC7_DMACH18: u32 = 0x00040000; // Timer0A / Timer1A
const SYSCTL_DC7_DMACH17: u32 = 0x00020000; // ADC0_SS3
const SYSCTL_DC7_DMACH16: u32 = 0x00010000; // ADC0_SS2
const SYSCTL_DC7_DMACH15: u32 = 0x00008000; // ADC0_SS1 / Timer2B
const SYSCTL_DC7_DMACH14: u32 = 0x00004000; // ADC0_SS0 / Timer2A
const SYSCTL_DC7_DMACH13: u32 = 0x00002000; // CAN0_TX / UART2_TX
const SYSCTL_DC7_DMACH12: u32 = 0x00001000; // CAN0_RX / UART2_RX
const SYSCTL_DC7_DMACH11: u32 = 0x00000800; // SSI0_TX / SSI1_TX
const SYSCTL_DC7_DMACH10: u32 = 0x00000400; // SSI0_RX / SSI1_RX
const SYSCTL_DC7_DMACH9: u32 = 0x00000200; // UART0_TX / UART1_TX
const SYSCTL_DC7_DMACH8: u32 = 0x00000100; // UART0_RX / UART1_RX
const SYSCTL_DC7_DMACH7: u32 = 0x00000080; // ETH_TX / Timer2B
const SYSCTL_DC7_DMACH6: u32 = 0x00000040; // ETH_RX / Timer2A
const SYSCTL_DC7_DMACH5: u32 = 0x00000020; // USB_EP3_TX / Timer2B
const SYSCTL_DC7_DMACH4: u32 = 0x00000010; // USB_EP3_RX / Timer2A
const SYSCTL_DC7_DMACH3: u32 = 0x00000008; // USB_EP2_TX / Timer3B
const SYSCTL_DC7_DMACH2: u32 = 0x00000004; // USB_EP2_RX / Timer3A
const SYSCTL_DC7_DMACH1: u32 = 0x00000002; // USB_EP1_TX / UART2_TX
const SYSCTL_DC7_DMACH0: u32 = 0x00000001; // USB_EP1_RX / UART2_RX

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC8 register.
//
// *****************************************************************************
const SYSCTL_DC8_ADC1AIN15: u32 = 0x80000000; // ADC Module 1 AIN15 Pin Present
const SYSCTL_DC8_ADC1AIN14: u32 = 0x40000000; // ADC Module 1 AIN14 Pin Present
const SYSCTL_DC8_ADC1AIN13: u32 = 0x20000000; // ADC Module 1 AIN13 Pin Present
const SYSCTL_DC8_ADC1AIN12: u32 = 0x10000000; // ADC Module 1 AIN12 Pin Present
const SYSCTL_DC8_ADC1AIN11: u32 = 0x08000000; // ADC Module 1 AIN11 Pin Present
const SYSCTL_DC8_ADC1AIN10: u32 = 0x04000000; // ADC Module 1 AIN10 Pin Present
const SYSCTL_DC8_ADC1AIN9: u32 = 0x02000000; // ADC Module 1 AIN9 Pin Present
const SYSCTL_DC8_ADC1AIN8: u32 = 0x01000000; // ADC Module 1 AIN8 Pin Present
const SYSCTL_DC8_ADC1AIN7: u32 = 0x00800000; // ADC Module 1 AIN7 Pin Present
const SYSCTL_DC8_ADC1AIN6: u32 = 0x00400000; // ADC Module 1 AIN6 Pin Present
const SYSCTL_DC8_ADC1AIN5: u32 = 0x00200000; // ADC Module 1 AIN5 Pin Present
const SYSCTL_DC8_ADC1AIN4: u32 = 0x00100000; // ADC Module 1 AIN4 Pin Present
const SYSCTL_DC8_ADC1AIN3: u32 = 0x00080000; // ADC Module 1 AIN3 Pin Present
const SYSCTL_DC8_ADC1AIN2: u32 = 0x00040000; // ADC Module 1 AIN2 Pin Present
const SYSCTL_DC8_ADC1AIN1: u32 = 0x00020000; // ADC Module 1 AIN1 Pin Present
const SYSCTL_DC8_ADC1AIN0: u32 = 0x00010000; // ADC Module 1 AIN0 Pin Present
const SYSCTL_DC8_ADC0AIN15: u32 = 0x00008000; // ADC Module 0 AIN15 Pin Present
const SYSCTL_DC8_ADC0AIN14: u32 = 0x00004000; // ADC Module 0 AIN14 Pin Present
const SYSCTL_DC8_ADC0AIN13: u32 = 0x00002000; // ADC Module 0 AIN13 Pin Present
const SYSCTL_DC8_ADC0AIN12: u32 = 0x00001000; // ADC Module 0 AIN12 Pin Present
const SYSCTL_DC8_ADC0AIN11: u32 = 0x00000800; // ADC Module 0 AIN11 Pin Present
const SYSCTL_DC8_ADC0AIN10: u32 = 0x00000400; // ADC Module 0 AIN10 Pin Present
const SYSCTL_DC8_ADC0AIN9: u32 = 0x00000200; // ADC Module 0 AIN9 Pin Present
const SYSCTL_DC8_ADC0AIN8: u32 = 0x00000100; // ADC Module 0 AIN8 Pin Present
const SYSCTL_DC8_ADC0AIN7: u32 = 0x00000080; // ADC Module 0 AIN7 Pin Present
const SYSCTL_DC8_ADC0AIN6: u32 = 0x00000040; // ADC Module 0 AIN6 Pin Present
const SYSCTL_DC8_ADC0AIN5: u32 = 0x00000020; // ADC Module 0 AIN5 Pin Present
const SYSCTL_DC8_ADC0AIN4: u32 = 0x00000010; // ADC Module 0 AIN4 Pin Present
const SYSCTL_DC8_ADC0AIN3: u32 = 0x00000008; // ADC Module 0 AIN3 Pin Present
const SYSCTL_DC8_ADC0AIN2: u32 = 0x00000004; // ADC Module 0 AIN2 Pin Present
const SYSCTL_DC8_ADC0AIN1: u32 = 0x00000002; // ADC Module 0 AIN1 Pin Present
const SYSCTL_DC8_ADC0AIN0: u32 = 0x00000001; // ADC Module 0 AIN0 Pin Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PBORCTL register.
//
// *****************************************************************************
const SYSCTL_PBORCTL_BORIOR: u32 = 0x00000002; // BOR Interrupt or Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRCR0 register.
//
// *****************************************************************************
const SYSCTL_SRCR0_WDT1: u32 = 0x10000000; // WDT1 Reset Control
const SYSCTL_SRCR0_CAN1: u32 = 0x02000000; // CAN1 Reset Control
const SYSCTL_SRCR0_CAN0: u32 = 0x01000000; // CAN0 Reset Control
const SYSCTL_SRCR0_PWM0: u32 = 0x00100000; // PWM Reset Control
const SYSCTL_SRCR0_ADC1: u32 = 0x00020000; // ADC1 Reset Control
const SYSCTL_SRCR0_ADC0: u32 = 0x00010000; // ADC0 Reset Control
const SYSCTL_SRCR0_HIB: u32 = 0x00000040; // HIB Reset Control
const SYSCTL_SRCR0_WDT0: u32 = 0x00000008; // WDT0 Reset Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRCR1 register.
//
// *****************************************************************************
const SYSCTL_SRCR1_COMP2: u32 = 0x04000000; // Analog Comp 2 Reset Control
const SYSCTL_SRCR1_COMP1: u32 = 0x02000000; // Analog Comp 1 Reset Control
const SYSCTL_SRCR1_COMP0: u32 = 0x01000000; // Analog Comp 0 Reset Control
const SYSCTL_SRCR1_TIMER3: u32 = 0x00080000; // Timer 3 Reset Control
const SYSCTL_SRCR1_TIMER2: u32 = 0x00040000; // Timer 2 Reset Control
const SYSCTL_SRCR1_TIMER1: u32 = 0x00020000; // Timer 1 Reset Control
const SYSCTL_SRCR1_TIMER0: u32 = 0x00010000; // Timer 0 Reset Control
const SYSCTL_SRCR1_I2C1: u32 = 0x00004000; // I2C1 Reset Control
const SYSCTL_SRCR1_I2C0: u32 = 0x00001000; // I2C0 Reset Control
const SYSCTL_SRCR1_QEI1: u32 = 0x00000200; // QEI1 Reset Control
const SYSCTL_SRCR1_QEI0: u32 = 0x00000100; // QEI0 Reset Control
const SYSCTL_SRCR1_SSI1: u32 = 0x00000020; // SSI1 Reset Control
const SYSCTL_SRCR1_SSI0: u32 = 0x00000010; // SSI0 Reset Control
const SYSCTL_SRCR1_UART2: u32 = 0x00000004; // UART2 Reset Control
const SYSCTL_SRCR1_UART1: u32 = 0x00000002; // UART1 Reset Control
const SYSCTL_SRCR1_UART0: u32 = 0x00000001; // UART0 Reset Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRCR2 register.
//
// *****************************************************************************
const SYSCTL_SRCR2_USB0: u32 = 0x00010000; // USB0 Reset Control
const SYSCTL_SRCR2_UDMA: u32 = 0x00002000; // Micro-DMA Reset Control
const SYSCTL_SRCR2_GPIOJ: u32 = 0x00000100; // Port J Reset Control
const SYSCTL_SRCR2_GPIOH: u32 = 0x00000080; // Port H Reset Control
const SYSCTL_SRCR2_GPIOG: u32 = 0x00000040; // Port G Reset Control
const SYSCTL_SRCR2_GPIOF: u32 = 0x00000020; // Port F Reset Control
const SYSCTL_SRCR2_GPIOE: u32 = 0x00000010; // Port E Reset Control
const SYSCTL_SRCR2_GPIOD: u32 = 0x00000008; // Port D Reset Control
const SYSCTL_SRCR2_GPIOC: u32 = 0x00000004; // Port C Reset Control
const SYSCTL_SRCR2_GPIOB: u32 = 0x00000002; // Port B Reset Control
const SYSCTL_SRCR2_GPIOA: u32 = 0x00000001; // Port A Reset Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RIS register.
//
// *****************************************************************************
const SYSCTL_RIS_MOSCPUPRIS: u32 = 0x00000100; // MOSC Power Up Raw Interrupt Status
const SYSCTL_RIS_USBPLLLRIS: u32 = 0x00000080; // USB PLL Lock Raw Interrupt Status
const SYSCTL_RIS_PLLLRIS: u32 = 0x00000040; // PLL Lock Raw Interrupt Status
const SYSCTL_RIS_MOFRIS: u32 = 0x00000008; // Main Oscillator Fault Raw Interrupt Status
const SYSCTL_RIS_BORRIS: u32 = 0x00000002; // Brown-Out Reset Raw Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_IMC register.
//
// *****************************************************************************
const SYSCTL_IMC_MOSCPUPIM: u32 = 0x00000100; // MOSC Power Up Interrupt Mask
const SYSCTL_IMC_USBPLLLIM: u32 = 0x00000080; // USB PLL Lock Interrupt Mask
const SYSCTL_IMC_PLLLIM: u32 = 0x00000040; // PLL Lock Interrupt Mask
const SYSCTL_IMC_MOFIM: u32 = 0x00000008; // Main Oscillator Fault Interrupt Mask
const SYSCTL_IMC_BORIM: u32 = 0x00000002; // Brown-Out Reset Interrupt Mask

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_MISC register.
//
// *****************************************************************************
const SYSCTL_MISC_MOSCPUPMIS: u32 = 0x00000100; // MOSC Power Up Masked Interrupt Status
const SYSCTL_MISC_USBPLLLMIS: u32 = 0x00000080; // USB PLL Lock Masked Interrupt Status
const SYSCTL_MISC_PLLLMIS: u32 = 0x00000040; // PLL Lock Masked Interrupt Status
const SYSCTL_MISC_MOFMIS: u32 = 0x00000008; // Main Oscillator Fault Masked Interrupt Status
const SYSCTL_MISC_BORMIS: u32 = 0x00000002; // BOR Masked Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RESC register.
//
// *****************************************************************************
const SYSCTL_RESC_MOSCFAIL: u32 = 0x00010000; // MOSC Failure Reset
const SYSCTL_RESC_WDT1: u32 = 0x00000020; // Watchdog Timer 1 Reset
const SYSCTL_RESC_SW: u32 = 0x00000010; // Software Reset
const SYSCTL_RESC_WDT0: u32 = 0x00000008; // Watchdog Timer 0 Reset
const SYSCTL_RESC_BOR: u32 = 0x00000004; // Brown-Out Reset
const SYSCTL_RESC_POR: u32 = 0x00000002; // Power-On Reset
const SYSCTL_RESC_EXT: u32 = 0x00000001; // External Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCC register.
//
// *****************************************************************************
const SYSCTL_RCC_ACG: u32 = 0x08000000; // Auto Clock Gating
const SYSCTL_RCC_SYSDIV_M: u32 = 0x07800000; // System Clock Divisor
const SYSCTL_RCC_USESYSDIV: u32 = 0x00400000; // Enable System Clock Divider
const SYSCTL_RCC_PWRDN: u32 = 0x00002000; // PLL Power Down
const SYSCTL_RCC_BYPASS: u32 = 0x00000800; // PLL Bypass
const SYSCTL_RCC_XTAL_M: u32 = 0x000007C0; // Crystal Value
const SYSCTL_RCC_XTAL_4MHZ: u32 = 0x00000180; // 4 MHz
const SYSCTL_RCC_XTAL_4_09MHZ: u32 = 0x000001C0; // 4.096 MHz
const SYSCTL_RCC_XTAL_4_91MHZ: u32 = 0x00000200; // 4.9152 MHz
const SYSCTL_RCC_XTAL_5MHZ: u32 = 0x00000240; // 5 MHz
const SYSCTL_RCC_XTAL_5_12MHZ: u32 = 0x00000280; // 5.12 MHz
const SYSCTL_RCC_XTAL_6MHZ: u32 = 0x000002C0; // 6 MHz
const SYSCTL_RCC_XTAL_6_14MHZ: u32 = 0x00000300; // 6.144 MHz
const SYSCTL_RCC_XTAL_7_37MHZ: u32 = 0x00000340; // 7.3728 MHz
const SYSCTL_RCC_XTAL_8MHZ: u32 = 0x00000380; // 8 MHz
const SYSCTL_RCC_XTAL_8_19MHZ: u32 = 0x000003C0; // 8.192 MHz
const SYSCTL_RCC_XTAL_10MHZ: u32 = 0x00000400; // 10 MHz
const SYSCTL_RCC_XTAL_12MHZ: u32 = 0x00000440; // 12 MHz
const SYSCTL_RCC_XTAL_12_2MHZ: u32 = 0x00000480; // 12.288 MHz
const SYSCTL_RCC_XTAL_13_5MHZ: u32 = 0x000004C0; // 13.56 MHz
const SYSCTL_RCC_XTAL_14_3MHZ: u32 = 0x00000500; // 14.31818 MHz
const SYSCTL_RCC_XTAL_16MHZ: u32 = 0x00000540; // 16 MHz
const SYSCTL_RCC_XTAL_16_3MHZ: u32 = 0x00000580; // 16.384 MHz
const SYSCTL_RCC_XTAL_18MHZ: u32 = 0x000005C0; // 18.0 MHz
const SYSCTL_RCC_XTAL_20MHZ: u32 = 0x00000600; // 20.0 MHz
const SYSCTL_RCC_XTAL_24MHZ: u32 = 0x00000640; // 24.0 MHz
const SYSCTL_RCC_XTAL_25MHZ: u32 = 0x00000680; // 25.0 MHz
const SYSCTL_RCC_OSCSRC_M: u32 = 0x00000030; // Oscillator Source
const SYSCTL_RCC_OSCSRC_MAIN: u32 = 0x00000000; // MOSC
const SYSCTL_RCC_OSCSRC_INT: u32 = 0x00000010; // IOSC
const SYSCTL_RCC_OSCSRC_INT4: u32 = 0x00000020; // IOSC/4
const SYSCTL_RCC_OSCSRC_30: u32 = 0x00000030; // 30 kHz
const SYSCTL_RCC_IOSCDIS: u32 = 0x00000002; // Internal Oscillator Disable
const SYSCTL_RCC_MOSCDIS: u32 = 0x00000001; // Main Oscillator Disable
const SYSCTL_RCC_SYSDIV_S: u32 = 23;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_GPIOHBCTL
// register.
//
// *****************************************************************************
const SYSCTL_GPIOHBCTL_PORTF: u32 = 0x00000020; // Port F Advanced High-Performance Bus
const SYSCTL_GPIOHBCTL_PORTE: u32 = 0x00000010; // Port E Advanced High-Performance Bus
const SYSCTL_GPIOHBCTL_PORTD: u32 = 0x00000008; // Port D Advanced High-Performance Bus
const SYSCTL_GPIOHBCTL_PORTC: u32 = 0x00000004; // Port C Advanced High-Performance Bus
const SYSCTL_GPIOHBCTL_PORTB: u32 = 0x00000002; // Port B Advanced High-Performance Bus
const SYSCTL_GPIOHBCTL_PORTA: u32 = 0x00000001; // Port A Advanced High-Performance Bus

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCC2 register.
//
// *****************************************************************************
const SYSCTL_RCC2_USERCC2: u32 = 0x80000000; // Use RCC2
const SYSCTL_RCC2_DIV400: u32 = 0x40000000; // Divide PLL as 400 MHz vs. 200 MHz
const SYSCTL_RCC2_SYSDIV2_M: u32 = 0x1F800000; // System Clock Divisor 2
const SYSCTL_RCC2_SYSDIV2_2: u32 = 0x00800000; // System clock /2
const SYSCTL_RCC2_SYSDIV2_3: u32 = 0x01000000; // System clock /3
const SYSCTL_RCC2_SYSDIV2_4: u32 = 0x01800000; // System clock /4
const SYSCTL_RCC2_SYSDIV2_5: u32 = 0x02000000; // System clock /5
const SYSCTL_RCC2_SYSDIV2_6: u32 = 0x02800000; // System clock /6
const SYSCTL_RCC2_SYSDIV2_7: u32 = 0x03000000; // System clock /7
const SYSCTL_RCC2_SYSDIV2_8: u32 = 0x03800000; // System clock /8
const SYSCTL_RCC2_SYSDIV2_9: u32 = 0x04000000; // System clock /9
const SYSCTL_RCC2_SYSDIV2_10: u32 = 0x04800000; // System clock /10
const SYSCTL_RCC2_SYSDIV2_11: u32 = 0x05000000; // System clock /11
const SYSCTL_RCC2_SYSDIV2_12: u32 = 0x05800000; // System clock /12
const SYSCTL_RCC2_SYSDIV2_13: u32 = 0x06000000; // System clock /13
const SYSCTL_RCC2_SYSDIV2_14: u32 = 0x06800000; // System clock /14
const SYSCTL_RCC2_SYSDIV2_15: u32 = 0x07000000; // System clock /15
const SYSCTL_RCC2_SYSDIV2_16: u32 = 0x07800000; // System clock /16
const SYSCTL_RCC2_SYSDIV2_17: u32 = 0x08000000; // System clock /17
const SYSCTL_RCC2_SYSDIV2_18: u32 = 0x08800000; // System clock /18
const SYSCTL_RCC2_SYSDIV2_19: u32 = 0x09000000; // System clock /19
const SYSCTL_RCC2_SYSDIV2_20: u32 = 0x09800000; // System clock /20
const SYSCTL_RCC2_SYSDIV2_21: u32 = 0x0A000000; // System clock /21
const SYSCTL_RCC2_SYSDIV2_22: u32 = 0x0A800000; // System clock /22
const SYSCTL_RCC2_SYSDIV2_23: u32 = 0x0B000000; // System clock /23
const SYSCTL_RCC2_SYSDIV2_24: u32 = 0x0B800000; // System clock /24
const SYSCTL_RCC2_SYSDIV2_25: u32 = 0x0C000000; // System clock /25
const SYSCTL_RCC2_SYSDIV2_26: u32 = 0x0C800000; // System clock /26
const SYSCTL_RCC2_SYSDIV2_27: u32 = 0x0D000000; // System clock /27
const SYSCTL_RCC2_SYSDIV2_28: u32 = 0x0D800000; // System clock /28
const SYSCTL_RCC2_SYSDIV2_29: u32 = 0x0E000000; // System clock /29
const SYSCTL_RCC2_SYSDIV2_30: u32 = 0x0E800000; // System clock /30
const SYSCTL_RCC2_SYSDIV2_31: u32 = 0x0F000000; // System clock /31
const SYSCTL_RCC2_SYSDIV2_32: u32 = 0x0F800000; // System clock /32
const SYSCTL_RCC2_SYSDIV2_33: u32 = 0x10000000; // System clock /33
const SYSCTL_RCC2_SYSDIV2_34: u32 = 0x10800000; // System clock /34
const SYSCTL_RCC2_SYSDIV2_35: u32 = 0x11000000; // System clock /35
const SYSCTL_RCC2_SYSDIV2_36: u32 = 0x11800000; // System clock /36
const SYSCTL_RCC2_SYSDIV2_37: u32 = 0x12000000; // System clock /37
const SYSCTL_RCC2_SYSDIV2_38: u32 = 0x12800000; // System clock /38
const SYSCTL_RCC2_SYSDIV2_39: u32 = 0x13000000; // System clock /39
const SYSCTL_RCC2_SYSDIV2_40: u32 = 0x13800000; // System clock /40
const SYSCTL_RCC2_SYSDIV2_41: u32 = 0x14000000; // System clock /41
const SYSCTL_RCC2_SYSDIV2_42: u32 = 0x14800000; // System clock /42
const SYSCTL_RCC2_SYSDIV2_43: u32 = 0x15000000; // System clock /43
const SYSCTL_RCC2_SYSDIV2_44: u32 = 0x15800000; // System clock /44
const SYSCTL_RCC2_SYSDIV2_45: u32 = 0x16000000; // System clock /45
const SYSCTL_RCC2_SYSDIV2_46: u32 = 0x16800000; // System clock /46
const SYSCTL_RCC2_SYSDIV2_47: u32 = 0x17000000; // System clock /47
const SYSCTL_RCC2_SYSDIV2_48: u32 = 0x17800000; // System clock /48
const SYSCTL_RCC2_SYSDIV2_49: u32 = 0x18000000; // System clock /49
const SYSCTL_RCC2_SYSDIV2_50: u32 = 0x18800000; // System clock /50
const SYSCTL_RCC2_SYSDIV2_51: u32 = 0x19000000; // System clock /51
const SYSCTL_RCC2_SYSDIV2_52: u32 = 0x19800000; // System clock /52
const SYSCTL_RCC2_SYSDIV2_53: u32 = 0x1A000000; // System clock /53
const SYSCTL_RCC2_SYSDIV2_54: u32 = 0x1A800000; // System clock /54
const SYSCTL_RCC2_SYSDIV2_55: u32 = 0x1B000000; // System clock /55
const SYSCTL_RCC2_SYSDIV2_56: u32 = 0x1B800000; // System clock /56
const SYSCTL_RCC2_SYSDIV2_57: u32 = 0x1C000000; // System clock /57
const SYSCTL_RCC2_SYSDIV2_58: u32 = 0x1C800000; // System clock /58
const SYSCTL_RCC2_SYSDIV2_59: u32 = 0x1D000000; // System clock /59
const SYSCTL_RCC2_SYSDIV2_60: u32 = 0x1D800000; // System clock /60
const SYSCTL_RCC2_SYSDIV2_61: u32 = 0x1E000000; // System clock /61
const SYSCTL_RCC2_SYSDIV2_62: u32 = 0x1E800000; // System clock /62
const SYSCTL_RCC2_SYSDIV2_63: u32 = 0x1F000000; // System clock /63
const SYSCTL_RCC2_SYSDIV2_64: u32 = 0x1F800000; // System clock /64
const SYSCTL_RCC2_SYSDIV2LSB: u32 = 0x00400000; // Additional LSB for SYSDIV2
const SYSCTL_RCC2_USBPWRDN: u32 = 0x00004000; // Power-Down USB PLL
const SYSCTL_RCC2_PWRDN2: u32 = 0x00002000; // Power-Down PLL 2
const SYSCTL_RCC2_BYPASS2: u32 = 0x00000800; // PLL Bypass 2
const SYSCTL_RCC2_OSCSRC2_M: u32 = 0x00000070; // Oscillator Source 2
const SYSCTL_RCC2_OSCSRC2_MO: u32 = 0x00000000; // MOSC
const SYSCTL_RCC2_OSCSRC2_IO: u32 = 0x00000010; // PIOSC
const SYSCTL_RCC2_OSCSRC2_IO4: u32 = 0x00000020; // PIOSC/4
const SYSCTL_RCC2_OSCSRC2_30: u32 = 0x00000030; // 30 kHz
const SYSCTL_RCC2_OSCSRC2_32: u32 = 0x00000070; // 32.768 kHz
const SYSCTL_RCC2_SYSDIV2_S: u32 = 23;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_MOSCCTL register.
//
// *****************************************************************************
const SYSCTL_MOSCCTL_NOXTAL: u32 = 0x00000004; // No Crystal Connected
const SYSCTL_MOSCCTL_MOSCIM: u32 = 0x00000002; // MOSC Failure Action
const SYSCTL_MOSCCTL_CVAL: u32 = 0x00000001; // Clock Validation for MOSC

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGC0 register.
//
// *****************************************************************************
const SYSCTL_RCGC0_WDT1: u32 = 0x10000000; // WDT1 Clock Gating Control
const SYSCTL_RCGC0_CAN1: u32 = 0x02000000; // CAN1 Clock Gating Control
const SYSCTL_RCGC0_CAN0: u32 = 0x01000000; // CAN0 Clock Gating Control
const SYSCTL_RCGC0_PWM0: u32 = 0x00100000; // PWM Clock Gating Control
const SYSCTL_RCGC0_ADC1: u32 = 0x00020000; // ADC1 Clock Gating Control
const SYSCTL_RCGC0_ADC0: u32 = 0x00010000; // ADC0 Clock Gating Control
const SYSCTL_RCGC0_ADC1SPD_M: u32 = 0x00000C00; // ADC1 Sample Speed
const SYSCTL_RCGC0_ADC1SPD_125K: u32 = 0x00000000; // 125K samples/second
const SYSCTL_RCGC0_ADC1SPD_250K: u32 = 0x00000400; // 250K samples/second
const SYSCTL_RCGC0_ADC1SPD_500K: u32 = 0x00000800; // 500K samples/second
const SYSCTL_RCGC0_ADC1SPD_1M: u32 = 0x00000C00; // 1M samples/second
const SYSCTL_RCGC0_ADC0SPD_M: u32 = 0x00000300; // ADC0 Sample Speed
const SYSCTL_RCGC0_ADC0SPD_125K: u32 = 0x00000000; // 125K samples/second
const SYSCTL_RCGC0_ADC0SPD_250K: u32 = 0x00000100; // 250K samples/second
const SYSCTL_RCGC0_ADC0SPD_500K: u32 = 0x00000200; // 500K samples/second
const SYSCTL_RCGC0_ADC0SPD_1M: u32 = 0x00000300; // 1M samples/second
const SYSCTL_RCGC0_HIB: u32 = 0x00000040; // HIB Clock Gating Control
const SYSCTL_RCGC0_WDT0: u32 = 0x00000008; // WDT0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGC1 register.
//
// *****************************************************************************
const SYSCTL_RCGC1_COMP2: u32 = 0x04000000; // Analog Comparator 2 Clock Gating
const SYSCTL_RCGC1_COMP1: u32 = 0x02000000; // Analog Comparator 1 Clock Gating
const SYSCTL_RCGC1_COMP0: u32 = 0x01000000; // Analog Comparator 0 Clock Gating
const SYSCTL_RCGC1_TIMER3: u32 = 0x00080000; // Timer 3 Clock Gating Control
const SYSCTL_RCGC1_TIMER2: u32 = 0x00040000; // Timer 2 Clock Gating Control
const SYSCTL_RCGC1_TIMER1: u32 = 0x00020000; // Timer 1 Clock Gating Control
const SYSCTL_RCGC1_TIMER0: u32 = 0x00010000; // Timer 0 Clock Gating Control
const SYSCTL_RCGC1_I2C1: u32 = 0x00004000; // I2C1 Clock Gating Control
const SYSCTL_RCGC1_I2C0: u32 = 0x00001000; // I2C0 Clock Gating Control
const SYSCTL_RCGC1_QEI1: u32 = 0x00000200; // QEI1 Clock Gating Control
const SYSCTL_RCGC1_QEI0: u32 = 0x00000100; // QEI0 Clock Gating Control
const SYSCTL_RCGC1_SSI1: u32 = 0x00000020; // SSI1 Clock Gating Control
const SYSCTL_RCGC1_SSI0: u32 = 0x00000010; // SSI0 Clock Gating Control
const SYSCTL_RCGC1_UART2: u32 = 0x00000004; // UART2 Clock Gating Control
const SYSCTL_RCGC1_UART1: u32 = 0x00000002; // UART1 Clock Gating Control
const SYSCTL_RCGC1_UART0: u32 = 0x00000001; // UART0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGC2 register.
//
// *****************************************************************************
const SYSCTL_RCGC2_USB0: u32 = 0x00010000; // USB0 Clock Gating Control
const SYSCTL_RCGC2_UDMA: u32 = 0x00002000; // Micro-DMA Clock Gating Control
const SYSCTL_RCGC2_GPIOJ: u32 = 0x00000100; // Port J Clock Gating Control
const SYSCTL_RCGC2_GPIOH: u32 = 0x00000080; // Port H Clock Gating Control
const SYSCTL_RCGC2_GPIOG: u32 = 0x00000040; // Port G Clock Gating Control
const SYSCTL_RCGC2_GPIOF: u32 = 0x00000020; // Port F Clock Gating Control
const SYSCTL_RCGC2_GPIOE: u32 = 0x00000010; // Port E Clock Gating Control
const SYSCTL_RCGC2_GPIOD: u32 = 0x00000008; // Port D Clock Gating Control
const SYSCTL_RCGC2_GPIOC: u32 = 0x00000004; // Port C Clock Gating Control
const SYSCTL_RCGC2_GPIOB: u32 = 0x00000002; // Port B Clock Gating Control
const SYSCTL_RCGC2_GPIOA: u32 = 0x00000001; // Port A Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGC0 register.
//
// *****************************************************************************
const SYSCTL_SCGC0_WDT1: u32 = 0x10000000; // WDT1 Clock Gating Control
const SYSCTL_SCGC0_CAN1: u32 = 0x02000000; // CAN1 Clock Gating Control
const SYSCTL_SCGC0_CAN0: u32 = 0x01000000; // CAN0 Clock Gating Control
const SYSCTL_SCGC0_PWM0: u32 = 0x00100000; // PWM Clock Gating Control
const SYSCTL_SCGC0_ADC1: u32 = 0x00020000; // ADC1 Clock Gating Control
const SYSCTL_SCGC0_ADC0: u32 = 0x00010000; // ADC0 Clock Gating Control
const SYSCTL_SCGC0_HIB: u32 = 0x00000040; // HIB Clock Gating Control
const SYSCTL_SCGC0_WDT0: u32 = 0x00000008; // WDT0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGC1 register.
//
// *****************************************************************************
const SYSCTL_SCGC1_COMP2: u32 = 0x04000000; // Analog Comparator 2 Clock Gating
const SYSCTL_SCGC1_COMP1: u32 = 0x02000000; // Analog Comparator 1 Clock Gating
const SYSCTL_SCGC1_COMP0: u32 = 0x01000000; // Analog Comparator 0 Clock Gating
const SYSCTL_SCGC1_TIMER3: u32 = 0x00080000; // Timer 3 Clock Gating Control
const SYSCTL_SCGC1_TIMER2: u32 = 0x00040000; // Timer 2 Clock Gating Control
const SYSCTL_SCGC1_TIMER1: u32 = 0x00020000; // Timer 1 Clock Gating Control
const SYSCTL_SCGC1_TIMER0: u32 = 0x00010000; // Timer 0 Clock Gating Control
const SYSCTL_SCGC1_I2C1: u32 = 0x00004000; // I2C1 Clock Gating Control
const SYSCTL_SCGC1_I2C0: u32 = 0x00001000; // I2C0 Clock Gating Control
const SYSCTL_SCGC1_QEI1: u32 = 0x00000200; // QEI1 Clock Gating Control
const SYSCTL_SCGC1_QEI0: u32 = 0x00000100; // QEI0 Clock Gating Control
const SYSCTL_SCGC1_SSI1: u32 = 0x00000020; // SSI1 Clock Gating Control
const SYSCTL_SCGC1_SSI0: u32 = 0x00000010; // SSI0 Clock Gating Control
const SYSCTL_SCGC1_UART2: u32 = 0x00000004; // UART2 Clock Gating Control
const SYSCTL_SCGC1_UART1: u32 = 0x00000002; // UART1 Clock Gating Control
const SYSCTL_SCGC1_UART0: u32 = 0x00000001; // UART0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGC2 register.
//
// *****************************************************************************
const SYSCTL_SCGC2_USB0: u32 = 0x00010000; // USB0 Clock Gating Control
const SYSCTL_SCGC2_UDMA: u32 = 0x00002000; // Micro-DMA Clock Gating Control
const SYSCTL_SCGC2_GPIOJ: u32 = 0x00000100; // Port J Clock Gating Control
const SYSCTL_SCGC2_GPIOH: u32 = 0x00000080; // Port H Clock Gating Control
const SYSCTL_SCGC2_GPIOG: u32 = 0x00000040; // Port G Clock Gating Control
const SYSCTL_SCGC2_GPIOF: u32 = 0x00000020; // Port F Clock Gating Control
const SYSCTL_SCGC2_GPIOE: u32 = 0x00000010; // Port E Clock Gating Control
const SYSCTL_SCGC2_GPIOD: u32 = 0x00000008; // Port D Clock Gating Control
const SYSCTL_SCGC2_GPIOC: u32 = 0x00000004; // Port C Clock Gating Control
const SYSCTL_SCGC2_GPIOB: u32 = 0x00000002; // Port B Clock Gating Control
const SYSCTL_SCGC2_GPIOA: u32 = 0x00000001; // Port A Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGC0 register.
//
// *****************************************************************************
const SYSCTL_DCGC0_WDT1: u32 = 0x10000000; // WDT1 Clock Gating Control
const SYSCTL_DCGC0_CAN1: u32 = 0x02000000; // CAN1 Clock Gating Control
const SYSCTL_DCGC0_CAN0: u32 = 0x01000000; // CAN0 Clock Gating Control
const SYSCTL_DCGC0_PWM0: u32 = 0x00100000; // PWM Clock Gating Control
const SYSCTL_DCGC0_ADC1: u32 = 0x00020000; // ADC1 Clock Gating Control
const SYSCTL_DCGC0_ADC0: u32 = 0x00010000; // ADC0 Clock Gating Control
const SYSCTL_DCGC0_HIB: u32 = 0x00000040; // HIB Clock Gating Control
const SYSCTL_DCGC0_WDT0: u32 = 0x00000008; // WDT0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGC1 register.
//
// *****************************************************************************
const SYSCTL_DCGC1_COMP2: u32 = 0x04000000; // Analog Comparator 2 Clock Gating
const SYSCTL_DCGC1_COMP1: u32 = 0x02000000; // Analog Comparator 1 Clock Gating
const SYSCTL_DCGC1_COMP0: u32 = 0x01000000; // Analog Comparator 0 Clock Gating
const SYSCTL_DCGC1_TIMER3: u32 = 0x00080000; // Timer 3 Clock Gating Control
const SYSCTL_DCGC1_TIMER2: u32 = 0x00040000; // Timer 2 Clock Gating Control
const SYSCTL_DCGC1_TIMER1: u32 = 0x00020000; // Timer 1 Clock Gating Control
const SYSCTL_DCGC1_TIMER0: u32 = 0x00010000; // Timer 0 Clock Gating Control
const SYSCTL_DCGC1_I2C1: u32 = 0x00004000; // I2C1 Clock Gating Control
const SYSCTL_DCGC1_I2C0: u32 = 0x00001000; // I2C0 Clock Gating Control
const SYSCTL_DCGC1_QEI1: u32 = 0x00000200; // QEI1 Clock Gating Control
const SYSCTL_DCGC1_QEI0: u32 = 0x00000100; // QEI0 Clock Gating Control
const SYSCTL_DCGC1_SSI1: u32 = 0x00000020; // SSI1 Clock Gating Control
const SYSCTL_DCGC1_SSI0: u32 = 0x00000010; // SSI0 Clock Gating Control
const SYSCTL_DCGC1_UART2: u32 = 0x00000004; // UART2 Clock Gating Control
const SYSCTL_DCGC1_UART1: u32 = 0x00000002; // UART1 Clock Gating Control
const SYSCTL_DCGC1_UART0: u32 = 0x00000001; // UART0 Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGC2 register.
//
// *****************************************************************************
const SYSCTL_DCGC2_USB0: u32 = 0x00010000; // USB0 Clock Gating Control
const SYSCTL_DCGC2_UDMA: u32 = 0x00002000; // Micro-DMA Clock Gating Control
const SYSCTL_DCGC2_GPIOJ: u32 = 0x00000100; // Port J Clock Gating Control
const SYSCTL_DCGC2_GPIOH: u32 = 0x00000080; // Port H Clock Gating Control
const SYSCTL_DCGC2_GPIOG: u32 = 0x00000040; // Port G Clock Gating Control
const SYSCTL_DCGC2_GPIOF: u32 = 0x00000020; // Port F Clock Gating Control
const SYSCTL_DCGC2_GPIOE: u32 = 0x00000010; // Port E Clock Gating Control
const SYSCTL_DCGC2_GPIOD: u32 = 0x00000008; // Port D Clock Gating Control
const SYSCTL_DCGC2_GPIOC: u32 = 0x00000004; // Port C Clock Gating Control
const SYSCTL_DCGC2_GPIOB: u32 = 0x00000002; // Port B Clock Gating Control
const SYSCTL_DCGC2_GPIOA: u32 = 0x00000001; // Port A Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DSLPCLKCFG
// register.
//
// *****************************************************************************
const SYSCTL_DSLPCLKCFG_D_M: u32 = 0x1F800000; // Divider Field Override
const SYSCTL_DSLPCLKCFG_D_1: u32 = 0x00000000; // System clock /1
const SYSCTL_DSLPCLKCFG_D_2: u32 = 0x00800000; // System clock /2
const SYSCTL_DSLPCLKCFG_D_3: u32 = 0x01000000; // System clock /3
const SYSCTL_DSLPCLKCFG_D_4: u32 = 0x01800000; // System clock /4
const SYSCTL_DSLPCLKCFG_D_64: u32 = 0x1F800000; // System clock /64
const SYSCTL_DSLPCLKCFG_O_M: u32 = 0x00000070; // Clock Source
const SYSCTL_DSLPCLKCFG_O_IGN: u32 = 0x00000000; // MOSC
const SYSCTL_DSLPCLKCFG_O_IO: u32 = 0x00000010; // PIOSC
const SYSCTL_DSLPCLKCFG_O_30: u32 = 0x00000030; // 30 kHz
const SYSCTL_DSLPCLKCFG_O_32: u32 = 0x00000070; // 32.768 kHz

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SYSPROP register.
//
// *****************************************************************************
const SYSCTL_SYSPROP_FPU: u32 = 0x00000001; // FPU Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PIOSCCAL
// register.
//
// *****************************************************************************
const SYSCTL_PIOSCCAL_UTEN: u32 = 0x80000000; // Use User Trim Value
const SYSCTL_PIOSCCAL_CAL: u32 = 0x00000200; // Start Calibration
const SYSCTL_PIOSCCAL_UPDATE: u32 = 0x00000100; // Update Trim
const SYSCTL_PIOSCCAL_UT_M: u32 = 0x0000007F; // User Trim Value
const SYSCTL_PIOSCCAL_UT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PIOSCSTAT
// register.
//
// *****************************************************************************
const SYSCTL_PIOSCSTAT_DT_M: u32 = 0x007F0000; // Default Trim Value
const SYSCTL_PIOSCSTAT_CR_M: u32 = 0x00000300; // Calibration Result
const SYSCTL_PIOSCSTAT_CRNONE: u32 = 0x00000000; // Calibration has not been attempted
const SYSCTL_PIOSCSTAT_CRPASS: u32 = 0x00000100; // The last calibration operation completed to meet 1% accuracy
const SYSCTL_PIOSCSTAT_CRFAIL: u32 = 0x00000200; // The last calibration operation failed to meet 1% accuracy
const SYSCTL_PIOSCSTAT_CT_M: u32 = 0x0000007F; // Calibration Trim Value
const SYSCTL_PIOSCSTAT_DT_S: u32 = 16;
const SYSCTL_PIOSCSTAT_CT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PLLFREQ0
// register.
//
// *****************************************************************************
const SYSCTL_PLLFREQ0_MFRAC_M: u32 = 0x000FFC00; // PLL M Fractional Value
const SYSCTL_PLLFREQ0_MINT_M: u32 = 0x000003FF; // PLL M Integer Value
const SYSCTL_PLLFREQ0_MFRAC_S: u32 = 10;
const SYSCTL_PLLFREQ0_MINT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PLLFREQ1
// register.
//
// *****************************************************************************
const SYSCTL_PLLFREQ1_Q_M: u32 = 0x00001F00; // PLL Q Value
const SYSCTL_PLLFREQ1_N_M: u32 = 0x0000001F; // PLL N Value
const SYSCTL_PLLFREQ1_Q_S: u32 = 8;
const SYSCTL_PLLFREQ1_N_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PLLSTAT register.
//
// *****************************************************************************
const SYSCTL_PLLSTAT_LOCK: u32 = 0x00000001; // PLL Lock

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DC9 register.
//
// *****************************************************************************
const SYSCTL_DC9_ADC1DC7: u32 = 0x00800000; // ADC1 DC7 Present
const SYSCTL_DC9_ADC1DC6: u32 = 0x00400000; // ADC1 DC6 Present
const SYSCTL_DC9_ADC1DC5: u32 = 0x00200000; // ADC1 DC5 Present
const SYSCTL_DC9_ADC1DC4: u32 = 0x00100000; // ADC1 DC4 Present
const SYSCTL_DC9_ADC1DC3: u32 = 0x00080000; // ADC1 DC3 Present
const SYSCTL_DC9_ADC1DC2: u32 = 0x00040000; // ADC1 DC2 Present
const SYSCTL_DC9_ADC1DC1: u32 = 0x00020000; // ADC1 DC1 Present
const SYSCTL_DC9_ADC1DC0: u32 = 0x00010000; // ADC1 DC0 Present
const SYSCTL_DC9_ADC0DC7: u32 = 0x00000080; // ADC0 DC7 Present
const SYSCTL_DC9_ADC0DC6: u32 = 0x00000040; // ADC0 DC6 Present
const SYSCTL_DC9_ADC0DC5: u32 = 0x00000020; // ADC0 DC5 Present
const SYSCTL_DC9_ADC0DC4: u32 = 0x00000010; // ADC0 DC4 Present
const SYSCTL_DC9_ADC0DC3: u32 = 0x00000008; // ADC0 DC3 Present
const SYSCTL_DC9_ADC0DC2: u32 = 0x00000004; // ADC0 DC2 Present
const SYSCTL_DC9_ADC0DC1: u32 = 0x00000002; // ADC0 DC1 Present
const SYSCTL_DC9_ADC0DC0: u32 = 0x00000001; // ADC0 DC0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_NVMSTAT register.
//
// *****************************************************************************
const SYSCTL_NVMSTAT_TPSW: u32 = 0x00000010; // Third Party Software Present
const SYSCTL_NVMSTAT_FWB: u32 = 0x00000001; // 32 Word Flash Write Buffer Active

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPWD register.
//
// *****************************************************************************
const SYSCTL_PPWD_P1: u32 = 0x00000002; // Watchdog Timer 1 Present
const SYSCTL_PPWD_P0: u32 = 0x00000001; // Watchdog Timer 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPTIMER register.
//
// *****************************************************************************
const SYSCTL_PPTIMER_P5: u32 = 0x00000020; // Timer 5 Present
const SYSCTL_PPTIMER_P4: u32 = 0x00000010; // Timer 4 Present
const SYSCTL_PPTIMER_P3: u32 = 0x00000008; // Timer 3 Present
const SYSCTL_PPTIMER_P2: u32 = 0x00000004; // Timer 2 Present
const SYSCTL_PPTIMER_P1: u32 = 0x00000002; // Timer 1 Present
const SYSCTL_PPTIMER_P0: u32 = 0x00000001; // Timer 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPGPIO register.
//
// *****************************************************************************
const SYSCTL_PPGPIO_P14: u32 = 0x00004000; // GPIO Port Q Present
const SYSCTL_PPGPIO_P13: u32 = 0x00002000; // GPIO Port P Present
const SYSCTL_PPGPIO_P12: u32 = 0x00001000; // GPIO Port N Present
const SYSCTL_PPGPIO_P11: u32 = 0x00000800; // GPIO Port M Present
const SYSCTL_PPGPIO_P10: u32 = 0x00000400; // GPIO Port L Present
const SYSCTL_PPGPIO_P9: u32 = 0x00000200; // GPIO Port K Present
const SYSCTL_PPGPIO_P8: u32 = 0x00000100; // GPIO Port J Present
const SYSCTL_PPGPIO_P7: u32 = 0x00000080; // GPIO Port H Present
const SYSCTL_PPGPIO_P6: u32 = 0x00000040; // GPIO Port G Present
const SYSCTL_PPGPIO_P5: u32 = 0x00000020; // GPIO Port F Present
const SYSCTL_PPGPIO_P4: u32 = 0x00000010; // GPIO Port E Present
const SYSCTL_PPGPIO_P3: u32 = 0x00000008; // GPIO Port D Present
const SYSCTL_PPGPIO_P2: u32 = 0x00000004; // GPIO Port C Present
const SYSCTL_PPGPIO_P1: u32 = 0x00000002; // GPIO Port B Present
const SYSCTL_PPGPIO_P0: u32 = 0x00000001; // GPIO Port A Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPDMA register.
//
// *****************************************************************************
const SYSCTL_PPDMA_P0: u32 = 0x00000001; // uDMA Module Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPHIB register.
//
// *****************************************************************************
const SYSCTL_PPHIB_P0: u32 = 0x00000001; // Hibernation Module Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPUART register.
//
// *****************************************************************************
const SYSCTL_PPUART_P7: u32 = 0x00000080; // UART Module 7 Present
const SYSCTL_PPUART_P6: u32 = 0x00000040; // UART Module 6 Present
const SYSCTL_PPUART_P5: u32 = 0x00000020; // UART Module 5 Present
const SYSCTL_PPUART_P4: u32 = 0x00000010; // UART Module 4 Present
const SYSCTL_PPUART_P3: u32 = 0x00000008; // UART Module 3 Present
const SYSCTL_PPUART_P2: u32 = 0x00000004; // UART Module 2 Present
const SYSCTL_PPUART_P1: u32 = 0x00000002; // UART Module 1 Present
const SYSCTL_PPUART_P0: u32 = 0x00000001; // UART Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPSSI register.
//
// *****************************************************************************
const SYSCTL_PPSSI_P3: u32 = 0x00000008; // SSI Module 3 Present
const SYSCTL_PPSSI_P2: u32 = 0x00000004; // SSI Module 2 Present
const SYSCTL_PPSSI_P1: u32 = 0x00000002; // SSI Module 1 Present
const SYSCTL_PPSSI_P0: u32 = 0x00000001; // SSI Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPI2C register.
//
// *****************************************************************************
const SYSCTL_PPI2C_P5: u32 = 0x00000020; // I2C Module 5 Present
const SYSCTL_PPI2C_P4: u32 = 0x00000010; // I2C Module 4 Present
const SYSCTL_PPI2C_P3: u32 = 0x00000008; // I2C Module 3 Present
const SYSCTL_PPI2C_P2: u32 = 0x00000004; // I2C Module 2 Present
const SYSCTL_PPI2C_P1: u32 = 0x00000002; // I2C Module 1 Present
const SYSCTL_PPI2C_P0: u32 = 0x00000001; // I2C Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPUSB register.
//
// *****************************************************************************
const SYSCTL_PPUSB_P0: u32 = 0x00000001; // USB Module Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPCAN register.
//
// *****************************************************************************
const SYSCTL_PPCAN_P1: u32 = 0x00000002; // CAN Module 1 Present
const SYSCTL_PPCAN_P0: u32 = 0x00000001; // CAN Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPADC register.
//
// *****************************************************************************
const SYSCTL_PPADC_P1: u32 = 0x00000002; // ADC Module 1 Present
const SYSCTL_PPADC_P0: u32 = 0x00000001; // ADC Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPACMP register.
//
// *****************************************************************************
const SYSCTL_PPACMP_P0: u32 = 0x00000001; // Analog Comparator Module Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPPWM register.
//
// *****************************************************************************
const SYSCTL_PPPWM_P1: u32 = 0x00000002; // PWM Module 1 Present
const SYSCTL_PPPWM_P0: u32 = 0x00000001; // PWM Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPQEI register.
//
// *****************************************************************************
const SYSCTL_PPQEI_P1: u32 = 0x00000002; // QEI Module 1 Present
const SYSCTL_PPQEI_P0: u32 = 0x00000001; // QEI Module 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPEEPROM
// register.
//
// *****************************************************************************
const SYSCTL_PPEEPROM_P0: u32 = 0x00000001; // EEPROM Module Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PPWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_PPWTIMER_P5: u32 = 0x00000020; // Wide Timer 5 Present
const SYSCTL_PPWTIMER_P4: u32 = 0x00000010; // Wide Timer 4 Present
const SYSCTL_PPWTIMER_P3: u32 = 0x00000008; // Wide Timer 3 Present
const SYSCTL_PPWTIMER_P2: u32 = 0x00000004; // Wide Timer 2 Present
const SYSCTL_PPWTIMER_P1: u32 = 0x00000002; // Wide Timer 1 Present
const SYSCTL_PPWTIMER_P0: u32 = 0x00000001; // Wide Timer 0 Present

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRWD register.
//
// *****************************************************************************
const SYSCTL_SRWD_R1: u32 = 0x00000002; // Watchdog Timer 1 Software Reset
const SYSCTL_SRWD_R0: u32 = 0x00000001; // Watchdog Timer 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRTIMER register.
//
// *****************************************************************************
const SYSCTL_SRTIMER_R5: u32 = 0x00000020; // Timer 5 Software Reset
const SYSCTL_SRTIMER_R4: u32 = 0x00000010; // Timer 4 Software Reset
const SYSCTL_SRTIMER_R3: u32 = 0x00000008; // Timer 3 Software Reset
const SYSCTL_SRTIMER_R2: u32 = 0x00000004; // Timer 2 Software Reset
const SYSCTL_SRTIMER_R1: u32 = 0x00000002; // Timer 1 Software Reset
const SYSCTL_SRTIMER_R0: u32 = 0x00000001; // Timer 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRGPIO register.
//
// *****************************************************************************
const SYSCTL_SRGPIO_R14: u32 = 0x00004000; // GPIO Port Q Software Reset
const SYSCTL_SRGPIO_R13: u32 = 0x00002000; // GPIO Port P Software Reset
const SYSCTL_SRGPIO_R12: u32 = 0x00001000; // GPIO Port N Software Reset
const SYSCTL_SRGPIO_R11: u32 = 0x00000800; // GPIO Port M Software Reset
const SYSCTL_SRGPIO_R10: u32 = 0x00000400; // GPIO Port L Software Reset
const SYSCTL_SRGPIO_R9: u32 = 0x00000200; // GPIO Port K Software Reset
const SYSCTL_SRGPIO_R8: u32 = 0x00000100; // GPIO Port J Software Reset
const SYSCTL_SRGPIO_R7: u32 = 0x00000080; // GPIO Port H Software Reset
const SYSCTL_SRGPIO_R6: u32 = 0x00000040; // GPIO Port G Software Reset
const SYSCTL_SRGPIO_R5: u32 = 0x00000020; // GPIO Port F Software Reset
const SYSCTL_SRGPIO_R4: u32 = 0x00000010; // GPIO Port E Software Reset
const SYSCTL_SRGPIO_R3: u32 = 0x00000008; // GPIO Port D Software Reset
const SYSCTL_SRGPIO_R2: u32 = 0x00000004; // GPIO Port C Software Reset
const SYSCTL_SRGPIO_R1: u32 = 0x00000002; // GPIO Port B Software Reset
const SYSCTL_SRGPIO_R0: u32 = 0x00000001; // GPIO Port A Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRDMA register.
//
// *****************************************************************************
const SYSCTL_SRDMA_R0: u32 = 0x00000001; // uDMA Module Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRHIB register.
//
// *****************************************************************************
const SYSCTL_SRHIB_R0: u32 = 0x00000001; // Hibernation Module Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRUART register.
//
// *****************************************************************************
const SYSCTL_SRUART_R7: u32 = 0x00000080; // UART Module 7 Software Reset
const SYSCTL_SRUART_R6: u32 = 0x00000040; // UART Module 6 Software Reset
const SYSCTL_SRUART_R5: u32 = 0x00000020; // UART Module 5 Software Reset
const SYSCTL_SRUART_R4: u32 = 0x00000010; // UART Module 4 Software Reset
const SYSCTL_SRUART_R3: u32 = 0x00000008; // UART Module 3 Software Reset
const SYSCTL_SRUART_R2: u32 = 0x00000004; // UART Module 2 Software Reset
const SYSCTL_SRUART_R1: u32 = 0x00000002; // UART Module 1 Software Reset
const SYSCTL_SRUART_R0: u32 = 0x00000001; // UART Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRSSI register.
//
// *****************************************************************************
const SYSCTL_SRSSI_R3: u32 = 0x00000008; // SSI Module 3 Software Reset
const SYSCTL_SRSSI_R2: u32 = 0x00000004; // SSI Module 2 Software Reset
const SYSCTL_SRSSI_R1: u32 = 0x00000002; // SSI Module 1 Software Reset
const SYSCTL_SRSSI_R0: u32 = 0x00000001; // SSI Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRI2C register.
//
// *****************************************************************************
const SYSCTL_SRI2C_R5: u32 = 0x00000020; // I2C Module 5 Software Reset
const SYSCTL_SRI2C_R4: u32 = 0x00000010; // I2C Module 4 Software Reset
const SYSCTL_SRI2C_R3: u32 = 0x00000008; // I2C Module 3 Software Reset
const SYSCTL_SRI2C_R2: u32 = 0x00000004; // I2C Module 2 Software Reset
const SYSCTL_SRI2C_R1: u32 = 0x00000002; // I2C Module 1 Software Reset
const SYSCTL_SRI2C_R0: u32 = 0x00000001; // I2C Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRUSB register.
//
// *****************************************************************************
const SYSCTL_SRUSB_R0: u32 = 0x00000001; // USB Module Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRCAN register.
//
// *****************************************************************************
const SYSCTL_SRCAN_R1: u32 = 0x00000002; // CAN Module 1 Software Reset
const SYSCTL_SRCAN_R0: u32 = 0x00000001; // CAN Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRADC register.
//
// *****************************************************************************
const SYSCTL_SRADC_R1: u32 = 0x00000002; // ADC Module 1 Software Reset
const SYSCTL_SRADC_R0: u32 = 0x00000001; // ADC Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRACMP register.
//
// *****************************************************************************
const SYSCTL_SRACMP_R0: u32 = 0x00000001; // Analog Comparator Module 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SREEPROM
// register.
//
// *****************************************************************************
const SYSCTL_SREEPROM_R0: u32 = 0x00000001; // EEPROM Module Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SRWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_SRWTIMER_R5: u32 = 0x00000020; // Wide Timer 5 Software Reset
const SYSCTL_SRWTIMER_R4: u32 = 0x00000010; // Wide Timer 4 Software Reset
const SYSCTL_SRWTIMER_R3: u32 = 0x00000008; // Wide Timer 3 Software Reset
const SYSCTL_SRWTIMER_R2: u32 = 0x00000004; // Wide Timer 2 Software Reset
const SYSCTL_SRWTIMER_R1: u32 = 0x00000002; // Wide Timer 1 Software Reset
const SYSCTL_SRWTIMER_R0: u32 = 0x00000001; // Wide Timer 0 Software Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCWD register.
//
// *****************************************************************************
const SYSCTL_RCGCWD_R1: u32 = 0x00000002; // Watchdog Timer 1 Run Mode Clock Gating Control
const SYSCTL_RCGCWD_R0: u32 = 0x00000001; // Watchdog Timer 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCTIMER
// register.
//
// *****************************************************************************
const SYSCTL_RCGCTIMER_R5: u32 = 0x00000020; // Timer 5 Run Mode Clock Gating Control
const SYSCTL_RCGCTIMER_R4: u32 = 0x00000010; // Timer 4 Run Mode Clock Gating Control
const SYSCTL_RCGCTIMER_R3: u32 = 0x00000008; // Timer 3 Run Mode Clock Gating Control
const SYSCTL_RCGCTIMER_R2: u32 = 0x00000004; // Timer 2 Run Mode Clock Gating Control
const SYSCTL_RCGCTIMER_R1: u32 = 0x00000002; // Timer 1 Run Mode Clock Gating Control
const SYSCTL_RCGCTIMER_R0: u32 = 0x00000001; // Timer 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCGPIO
// register.
//
// *****************************************************************************
const SYSCTL_RCGCGPIO_R14: u32 = 0x00004000; // GPIO Port Q Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R13: u32 = 0x00002000; // GPIO Port P Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R12: u32 = 0x00001000; // GPIO Port N Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R11: u32 = 0x00000800; // GPIO Port M Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R10: u32 = 0x00000400; // GPIO Port L Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R9: u32 = 0x00000200; // GPIO Port K Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R8: u32 = 0x00000100; // GPIO Port J Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R7: u32 = 0x00000080; // GPIO Port H Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R6: u32 = 0x00000040; // GPIO Port G Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R5: u32 = 0x00000020; // GPIO Port F Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R4: u32 = 0x00000010; // GPIO Port E Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R3: u32 = 0x00000008; // GPIO Port D Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R2: u32 = 0x00000004; // GPIO Port C Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R1: u32 = 0x00000002; // GPIO Port B Run Mode Clock Gating Control
const SYSCTL_RCGCGPIO_R0: u32 = 0x00000001; // GPIO Port A Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCDMA register.
//
// *****************************************************************************
const SYSCTL_RCGCDMA_R0: u32 = 0x00000001; // uDMA Module Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCHIB register.
//
// *****************************************************************************
const SYSCTL_RCGCHIB_R0: u32 = 0x00000001; // Hibernation Module Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCUART
// register.
//
// *****************************************************************************
const SYSCTL_RCGCUART_R7: u32 = 0x00000080; // UART Module 7 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R6: u32 = 0x00000040; // UART Module 6 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R5: u32 = 0x00000020; // UART Module 5 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R4: u32 = 0x00000010; // UART Module 4 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R3: u32 = 0x00000008; // UART Module 3 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R2: u32 = 0x00000004; // UART Module 2 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R1: u32 = 0x00000002; // UART Module 1 Run Mode Clock Gating Control
const SYSCTL_RCGCUART_R0: u32 = 0x00000001; // UART Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCSSI register.
//
// *****************************************************************************
const SYSCTL_RCGCSSI_R3: u32 = 0x00000008; // SSI Module 3 Run Mode Clock Gating Control
const SYSCTL_RCGCSSI_R2: u32 = 0x00000004; // SSI Module 2 Run Mode Clock Gating Control
const SYSCTL_RCGCSSI_R1: u32 = 0x00000002; // SSI Module 1 Run Mode Clock Gating Control
const SYSCTL_RCGCSSI_R0: u32 = 0x00000001; // SSI Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCI2C register.
//
// *****************************************************************************
const SYSCTL_RCGCI2C_R5: u32 = 0x00000020; // I2C Module 5 Run Mode Clock Gating Control
const SYSCTL_RCGCI2C_R4: u32 = 0x00000010; // I2C Module 4 Run Mode Clock Gating Control
const SYSCTL_RCGCI2C_R3: u32 = 0x00000008; // I2C Module 3 Run Mode Clock Gating Control
const SYSCTL_RCGCI2C_R2: u32 = 0x00000004; // I2C Module 2 Run Mode Clock Gating Control
const SYSCTL_RCGCI2C_R1: u32 = 0x00000002; // I2C Module 1 Run Mode Clock Gating Control
const SYSCTL_RCGCI2C_R0: u32 = 0x00000001; // I2C Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCUSB register.
//
// *****************************************************************************
const SYSCTL_RCGCUSB_R0: u32 = 0x00000001; // USB Module Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCCAN register.
//
// *****************************************************************************
const SYSCTL_RCGCCAN_R1: u32 = 0x00000002; // CAN Module 1 Run Mode Clock Gating Control
const SYSCTL_RCGCCAN_R0: u32 = 0x00000001; // CAN Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCADC register.
//
// *****************************************************************************
const SYSCTL_RCGCADC_R1: u32 = 0x00000002; // ADC Module 1 Run Mode Clock Gating Control
const SYSCTL_RCGCADC_R0: u32 = 0x00000001; // ADC Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCACMP
// register.
//
// *****************************************************************************
const SYSCTL_RCGCACMP_R0: u32 = 0x00000001; // Analog Comparator Module 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCEEPROM
// register.
//
// *****************************************************************************
const SYSCTL_RCGCEEPROM_R0: u32 = 0x00000001; // EEPROM Module Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_RCGCWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_RCGCWTIMER_R5: u32 = 0x00000020; // Wide Timer 5 Run Mode Clock Gating Control
const SYSCTL_RCGCWTIMER_R4: u32 = 0x00000010; // Wide Timer 4 Run Mode Clock Gating Control
const SYSCTL_RCGCWTIMER_R3: u32 = 0x00000008; // Wide Timer 3 Run Mode Clock Gating Control
const SYSCTL_RCGCWTIMER_R2: u32 = 0x00000004; // Wide Timer 2 Run Mode Clock Gating Control
const SYSCTL_RCGCWTIMER_R1: u32 = 0x00000002; // Wide Timer 1 Run Mode Clock Gating Control
const SYSCTL_RCGCWTIMER_R0: u32 = 0x00000001; // Wide Timer 0 Run Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCWD register.
//
// *****************************************************************************
const SYSCTL_SCGCWD_S1: u32 = 0x00000002; // Watchdog Timer 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWD_S0: u32 = 0x00000001; // Watchdog Timer 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCTIMER
// register.
//
// *****************************************************************************
const SYSCTL_SCGCTIMER_S5: u32 = 0x00000020; // Timer 5 Sleep Mode Clock Gating Control
const SYSCTL_SCGCTIMER_S4: u32 = 0x00000010; // Timer 4 Sleep Mode Clock Gating Control
const SYSCTL_SCGCTIMER_S3: u32 = 0x00000008; // Timer 3 Sleep Mode Clock Gating Control
const SYSCTL_SCGCTIMER_S2: u32 = 0x00000004; // Timer 2 Sleep Mode Clock Gating Control
const SYSCTL_SCGCTIMER_S1: u32 = 0x00000002; // Timer 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCTIMER_S0: u32 = 0x00000001; // Timer 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCGPIO
// register.
//
// *****************************************************************************
const SYSCTL_SCGCGPIO_S14: u32 = 0x00004000; // GPIO Port Q Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S13: u32 = 0x00002000; // GPIO Port P Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S12: u32 = 0x00001000; // GPIO Port N Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S11: u32 = 0x00000800; // GPIO Port M Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S10: u32 = 0x00000400; // GPIO Port L Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S9: u32 = 0x00000200; // GPIO Port K Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S8: u32 = 0x00000100; // GPIO Port J Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S7: u32 = 0x00000080; // GPIO Port H Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S6: u32 = 0x00000040; // GPIO Port G Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S5: u32 = 0x00000020; // GPIO Port F Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S4: u32 = 0x00000010; // GPIO Port E Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S3: u32 = 0x00000008; // GPIO Port D Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S2: u32 = 0x00000004; // GPIO Port C Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S1: u32 = 0x00000002; // GPIO Port B Sleep Mode Clock Gating Control
const SYSCTL_SCGCGPIO_S0: u32 = 0x00000001; // GPIO Port A Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCDMA register.
//
// *****************************************************************************
const SYSCTL_SCGCDMA_S0: u32 = 0x00000001; // uDMA Module Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCHIB register.
//
// *****************************************************************************
const SYSCTL_SCGCHIB_S0: u32 = 0x00000001; // Hibernation Module Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCUART
// register.
//
// *****************************************************************************
const SYSCTL_SCGCUART_S7: u32 = 0x00000080; // UART Module 7 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S6: u32 = 0x00000040; // UART Module 6 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S5: u32 = 0x00000020; // UART Module 5 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S4: u32 = 0x00000010; // UART Module 4 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S3: u32 = 0x00000008; // UART Module 3 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S2: u32 = 0x00000004; // UART Module 2 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S1: u32 = 0x00000002; // UART Module 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCUART_S0: u32 = 0x00000001; // UART Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCSSI register.
//
// *****************************************************************************
const SYSCTL_SCGCSSI_S3: u32 = 0x00000008; // SSI Module 3 Sleep Mode Clock Gating Control
const SYSCTL_SCGCSSI_S2: u32 = 0x00000004; // SSI Module 2 Sleep Mode Clock Gating Control
const SYSCTL_SCGCSSI_S1: u32 = 0x00000002; // SSI Module 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCSSI_S0: u32 = 0x00000001; // SSI Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCI2C register.
//
// *****************************************************************************
const SYSCTL_SCGCI2C_S5: u32 = 0x00000020; // I2C Module 5 Sleep Mode Clock Gating Control
const SYSCTL_SCGCI2C_S4: u32 = 0x00000010; // I2C Module 4 Sleep Mode Clock Gating Control
const SYSCTL_SCGCI2C_S3: u32 = 0x00000008; // I2C Module 3 Sleep Mode Clock Gating Control
const SYSCTL_SCGCI2C_S2: u32 = 0x00000004; // I2C Module 2 Sleep Mode Clock Gating Control
const SYSCTL_SCGCI2C_S1: u32 = 0x00000002; // I2C Module 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCI2C_S0: u32 = 0x00000001; // I2C Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCUSB register.
//
// *****************************************************************************
const SYSCTL_SCGCUSB_S0: u32 = 0x00000001; // USB Module Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCCAN register.
//
// *****************************************************************************
const SYSCTL_SCGCCAN_S1: u32 = 0x00000002; // CAN Module 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCCAN_S0: u32 = 0x00000001; // CAN Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCADC register.
//
// *****************************************************************************
const SYSCTL_SCGCADC_S1: u32 = 0x00000002; // ADC Module 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCADC_S0: u32 = 0x00000001; // ADC Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCACMP
// register.
//
// *****************************************************************************
const SYSCTL_SCGCACMP_S0: u32 = 0x00000001; // Analog Comparator Module 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCEEPROM
// register.
//
// *****************************************************************************
const SYSCTL_SCGCEEPROM_S0: u32 = 0x00000001; // EEPROM Module Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_SCGCWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_SCGCWTIMER_S5: u32 = 0x00000020; // Wide Timer 5 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWTIMER_S4: u32 = 0x00000010; // Wide Timer 4 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWTIMER_S3: u32 = 0x00000008; // Wide Timer 3 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWTIMER_S2: u32 = 0x00000004; // Wide Timer 2 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWTIMER_S1: u32 = 0x00000002; // Wide Timer 1 Sleep Mode Clock Gating Control
const SYSCTL_SCGCWTIMER_S0: u32 = 0x00000001; // Wide Timer 0 Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCWD register.
//
// *****************************************************************************
const SYSCTL_DCGCWD_D1: u32 = 0x00000002; // Watchdog Timer 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWD_D0: u32 = 0x00000001; // Watchdog Timer 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCTIMER
// register.
//
// *****************************************************************************
const SYSCTL_DCGCTIMER_D5: u32 = 0x00000020; // Timer 5 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCTIMER_D4: u32 = 0x00000010; // Timer 4 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCTIMER_D3: u32 = 0x00000008; // Timer 3 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCTIMER_D2: u32 = 0x00000004; // Timer 2 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCTIMER_D1: u32 = 0x00000002; // Timer 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCTIMER_D0: u32 = 0x00000001; // Timer 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCGPIO
// register.
//
// *****************************************************************************
const SYSCTL_DCGCGPIO_D14: u32 = 0x00004000; // GPIO Port Q Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D13: u32 = 0x00002000; // GPIO Port P Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D12: u32 = 0x00001000; // GPIO Port N Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D11: u32 = 0x00000800; // GPIO Port M Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D10: u32 = 0x00000400; // GPIO Port L Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D9: u32 = 0x00000200; // GPIO Port K Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D8: u32 = 0x00000100; // GPIO Port J Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D7: u32 = 0x00000080; // GPIO Port H Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D6: u32 = 0x00000040; // GPIO Port G Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D5: u32 = 0x00000020; // GPIO Port F Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D4: u32 = 0x00000010; // GPIO Port E Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D3: u32 = 0x00000008; // GPIO Port D Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D2: u32 = 0x00000004; // GPIO Port C Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D1: u32 = 0x00000002; // GPIO Port B Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCGPIO_D0: u32 = 0x00000001; // GPIO Port A Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCDMA register.
//
// *****************************************************************************
const SYSCTL_DCGCDMA_D0: u32 = 0x00000001; // uDMA Module Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCHIB register.
//
// *****************************************************************************
const SYSCTL_DCGCHIB_D0: u32 = 0x00000001; // Hibernation Module Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCUART
// register.
//
// *****************************************************************************
const SYSCTL_DCGCUART_D7: u32 = 0x00000080; // UART Module 7 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D6: u32 = 0x00000040; // UART Module 6 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D5: u32 = 0x00000020; // UART Module 5 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D4: u32 = 0x00000010; // UART Module 4 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D3: u32 = 0x00000008; // UART Module 3 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D2: u32 = 0x00000004; // UART Module 2 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D1: u32 = 0x00000002; // UART Module 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCUART_D0: u32 = 0x00000001; // UART Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCSSI register.
//
// *****************************************************************************
const SYSCTL_DCGCSSI_D3: u32 = 0x00000008; // SSI Module 3 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCSSI_D2: u32 = 0x00000004; // SSI Module 2 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCSSI_D1: u32 = 0x00000002; // SSI Module 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCSSI_D0: u32 = 0x00000001; // SSI Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCI2C register.
//
// *****************************************************************************
const SYSCTL_DCGCI2C_D5: u32 = 0x00000020; // I2C Module 5 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCI2C_D4: u32 = 0x00000010; // I2C Module 4 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCI2C_D3: u32 = 0x00000008; // I2C Module 3 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCI2C_D2: u32 = 0x00000004; // I2C Module 2 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCI2C_D1: u32 = 0x00000002; // I2C Module 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCI2C_D0: u32 = 0x00000001; // I2C Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCUSB register.
//
// *****************************************************************************
const SYSCTL_DCGCUSB_D0: u32 = 0x00000001; // USB Module Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCCAN register.
//
// *****************************************************************************
const SYSCTL_DCGCCAN_D1: u32 = 0x00000002; // CAN Module 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCCAN_D0: u32 = 0x00000001; // CAN Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCADC register.
//
// *****************************************************************************
const SYSCTL_DCGCADC_D1: u32 = 0x00000002; // ADC Module 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCADC_D0: u32 = 0x00000001; // ADC Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCACMP
// register.
//
// *****************************************************************************
const SYSCTL_DCGCACMP_D0: u32 = 0x00000001; // Analog Comparator Module 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCEEPROM
// register.
//
// *****************************************************************************
const SYSCTL_DCGCEEPROM_D0: u32 = 0x00000001; // EEPROM Module Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_DCGCWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_DCGCWTIMER_D5: u32 = 0x00000020; // Wide Timer 5 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWTIMER_D4: u32 = 0x00000010; // Wide Timer 4 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWTIMER_D3: u32 = 0x00000008; // Wide Timer 3 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWTIMER_D2: u32 = 0x00000004; // Wide Timer 2 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWTIMER_D1: u32 = 0x00000002; // Wide Timer 1 Deep-Sleep Mode Clock Gating Control
const SYSCTL_DCGCWTIMER_D0: u32 = 0x00000001; // Wide Timer 0 Deep-Sleep Mode Clock Gating Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCWD register.
//
// *****************************************************************************
const SYSCTL_PCWD_P1: u32 = 0x00000002; // Watchdog Timer 1 Power Control
const SYSCTL_PCWD_P0: u32 = 0x00000001; // Watchdog Timer 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCTIMER register.
//
// *****************************************************************************
const SYSCTL_PCTIMER_P5: u32 = 0x00000020; // Timer 5 Power Control
const SYSCTL_PCTIMER_P4: u32 = 0x00000010; // Timer 4 Power Control
const SYSCTL_PCTIMER_P3: u32 = 0x00000008; // Timer 3 Power Control
const SYSCTL_PCTIMER_P2: u32 = 0x00000004; // Timer 2 Power Control
const SYSCTL_PCTIMER_P1: u32 = 0x00000002; // Timer 1 Power Control
const SYSCTL_PCTIMER_P0: u32 = 0x00000001; // Timer 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCGPIO register.
//
// *****************************************************************************
const SYSCTL_PCGPIO_P14: u32 = 0x00004000; // GPIO Port Q Power Control
const SYSCTL_PCGPIO_P13: u32 = 0x00002000; // GPIO Port P Power Control
const SYSCTL_PCGPIO_P12: u32 = 0x00001000; // GPIO Port N Power Control
const SYSCTL_PCGPIO_P11: u32 = 0x00000800; // GPIO Port M Power Control
const SYSCTL_PCGPIO_P10: u32 = 0x00000400; // GPIO Port L Power Control
const SYSCTL_PCGPIO_P9: u32 = 0x00000200; // GPIO Port K Power Control
const SYSCTL_PCGPIO_P8: u32 = 0x00000100; // GPIO Port J Power Control
const SYSCTL_PCGPIO_P7: u32 = 0x00000080; // GPIO Port H Power Control
const SYSCTL_PCGPIO_P6: u32 = 0x00000040; // GPIO Port G Power Control
const SYSCTL_PCGPIO_P5: u32 = 0x00000020; // GPIO Port F Power Control
const SYSCTL_PCGPIO_P4: u32 = 0x00000010; // GPIO Port E Power Control
const SYSCTL_PCGPIO_P3: u32 = 0x00000008; // GPIO Port D Power Control
const SYSCTL_PCGPIO_P2: u32 = 0x00000004; // GPIO Port C Power Control
const SYSCTL_PCGPIO_P1: u32 = 0x00000002; // GPIO Port B Power Control
const SYSCTL_PCGPIO_P0: u32 = 0x00000001; // GPIO Port A Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCDMA register.
//
// *****************************************************************************
const SYSCTL_PCDMA_P0: u32 = 0x00000001; // uDMA Module Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCHIB register.
//
// *****************************************************************************
const SYSCTL_PCHIB_P0: u32 = 0x00000001; // Hibernation Module Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCUART register.
//
// *****************************************************************************
const SYSCTL_PCUART_P7: u32 = 0x00000080; // UART Module 7 Power Control
const SYSCTL_PCUART_P6: u32 = 0x00000040; // UART Module 6 Power Control
const SYSCTL_PCUART_P5: u32 = 0x00000020; // UART Module 5 Power Control
const SYSCTL_PCUART_P4: u32 = 0x00000010; // UART Module 4 Power Control
const SYSCTL_PCUART_P3: u32 = 0x00000008; // UART Module 3 Power Control
const SYSCTL_PCUART_P2: u32 = 0x00000004; // UART Module 2 Power Control
const SYSCTL_PCUART_P1: u32 = 0x00000002; // UART Module 1 Power Control
const SYSCTL_PCUART_P0: u32 = 0x00000001; // UART Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCSSI register.
//
// *****************************************************************************
const SYSCTL_PCSSI_P3: u32 = 0x00000008; // SSI Module 3 Power Control
const SYSCTL_PCSSI_P2: u32 = 0x00000004; // SSI Module 2 Power Control
const SYSCTL_PCSSI_P1: u32 = 0x00000002; // SSI Module 1 Power Control
const SYSCTL_PCSSI_P0: u32 = 0x00000001; // SSI Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCI2C register.
//
// *****************************************************************************
const SYSCTL_PCI2C_P5: u32 = 0x00000020; // I2C Module 5 Power Control
const SYSCTL_PCI2C_P4: u32 = 0x00000010; // I2C Module 4 Power Control
const SYSCTL_PCI2C_P3: u32 = 0x00000008; // I2C Module 3 Power Control
const SYSCTL_PCI2C_P2: u32 = 0x00000004; // I2C Module 2 Power Control
const SYSCTL_PCI2C_P1: u32 = 0x00000002; // I2C Module 1 Power Control
const SYSCTL_PCI2C_P0: u32 = 0x00000001; // I2C Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCUSB register.
//
// *****************************************************************************
const SYSCTL_PCUSB_P0: u32 = 0x00000001; // USB Module Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCCAN register.
//
// *****************************************************************************
const SYSCTL_PCCAN_P1: u32 = 0x00000002; // CAN Module 1 Power Control
const SYSCTL_PCCAN_P0: u32 = 0x00000001; // CAN Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCADC register.
//
// *****************************************************************************
const SYSCTL_PCADC_P1: u32 = 0x00000002; // ADC Module 1 Power Control
const SYSCTL_PCADC_P0: u32 = 0x00000001; // ADC Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCACMP register.
//
// *****************************************************************************
const SYSCTL_PCACMP_P0: u32 = 0x00000001; // Analog Comparator Module 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCEEPROM
// register.
//
// *****************************************************************************
const SYSCTL_PCEEPROM_P0: u32 = 0x00000001; // EEPROM Module Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PCWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_PCWTIMER_P5: u32 = 0x00000020; // Wide Timer 5 Power Control
const SYSCTL_PCWTIMER_P4: u32 = 0x00000010; // Wide Timer 4 Power Control
const SYSCTL_PCWTIMER_P3: u32 = 0x00000008; // Wide Timer 3 Power Control
const SYSCTL_PCWTIMER_P2: u32 = 0x00000004; // Wide Timer 2 Power Control
const SYSCTL_PCWTIMER_P1: u32 = 0x00000002; // Wide Timer 1 Power Control
const SYSCTL_PCWTIMER_P0: u32 = 0x00000001; // Wide Timer 0 Power Control

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRWD register.
//
// *****************************************************************************
const SYSCTL_PRWD_R1: u32 = 0x00000002; // Watchdog Timer 1 Peripheral Ready
const SYSCTL_PRWD_R0: u32 = 0x00000001; // Watchdog Timer 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRTIMER register.
//
// *****************************************************************************
const SYSCTL_PRTIMER_R5: u32 = 0x00000020; // Timer 5 Peripheral Ready
const SYSCTL_PRTIMER_R4: u32 = 0x00000010; // Timer 4 Peripheral Ready
const SYSCTL_PRTIMER_R3: u32 = 0x00000008; // Timer 3 Peripheral Ready
const SYSCTL_PRTIMER_R2: u32 = 0x00000004; // Timer 2 Peripheral Ready
const SYSCTL_PRTIMER_R1: u32 = 0x00000002; // Timer 1 Peripheral Ready
const SYSCTL_PRTIMER_R0: u32 = 0x00000001; // Timer 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRGPIO register.
//
// *****************************************************************************
const SYSCTL_PRGPIO_R14: u32 = 0x00004000; // GPIO Port Q Peripheral Ready
const SYSCTL_PRGPIO_R13: u32 = 0x00002000; // GPIO Port P Peripheral Ready
const SYSCTL_PRGPIO_R12: u32 = 0x00001000; // GPIO Port N Peripheral Ready
const SYSCTL_PRGPIO_R11: u32 = 0x00000800; // GPIO Port M Peripheral Ready
const SYSCTL_PRGPIO_R10: u32 = 0x00000400; // GPIO Port L Peripheral Ready
const SYSCTL_PRGPIO_R9: u32 = 0x00000200; // GPIO Port K Peripheral Ready
const SYSCTL_PRGPIO_R8: u32 = 0x00000100; // GPIO Port J Peripheral Ready
const SYSCTL_PRGPIO_R7: u32 = 0x00000080; // GPIO Port H Peripheral Ready
const SYSCTL_PRGPIO_R6: u32 = 0x00000040; // GPIO Port G Peripheral Ready
const SYSCTL_PRGPIO_R5: u32 = 0x00000020; // GPIO Port F Peripheral Ready
const SYSCTL_PRGPIO_R4: u32 = 0x00000010; // GPIO Port E Peripheral Ready
const SYSCTL_PRGPIO_R3: u32 = 0x00000008; // GPIO Port D Peripheral Ready
const SYSCTL_PRGPIO_R2: u32 = 0x00000004; // GPIO Port C Peripheral Ready
const SYSCTL_PRGPIO_R1: u32 = 0x00000002; // GPIO Port B Peripheral Ready
const SYSCTL_PRGPIO_R0: u32 = 0x00000001; // GPIO Port A Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRDMA register.
//
// *****************************************************************************
const SYSCTL_PRDMA_R0: u32 = 0x00000001; // uDMA Module Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRHIB register.
//
// *****************************************************************************
const SYSCTL_PRHIB_R0: u32 = 0x00000001; // Hibernation Module Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRUART register.
//
// *****************************************************************************
const SYSCTL_PRUART_R7: u32 = 0x00000080; // UART Module 7 Peripheral Ready
const SYSCTL_PRUART_R6: u32 = 0x00000040; // UART Module 6 Peripheral Ready
const SYSCTL_PRUART_R5: u32 = 0x00000020; // UART Module 5 Peripheral Ready
const SYSCTL_PRUART_R4: u32 = 0x00000010; // UART Module 4 Peripheral Ready
const SYSCTL_PRUART_R3: u32 = 0x00000008; // UART Module 3 Peripheral Ready
const SYSCTL_PRUART_R2: u32 = 0x00000004; // UART Module 2 Peripheral Ready
const SYSCTL_PRUART_R1: u32 = 0x00000002; // UART Module 1 Peripheral Ready
const SYSCTL_PRUART_R0: u32 = 0x00000001; // UART Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRSSI register.
//
// *****************************************************************************
const SYSCTL_PRSSI_R3: u32 = 0x00000008; // SSI Module 3 Peripheral Ready
const SYSCTL_PRSSI_R2: u32 = 0x00000004; // SSI Module 2 Peripheral Ready
const SYSCTL_PRSSI_R1: u32 = 0x00000002; // SSI Module 1 Peripheral Ready
const SYSCTL_PRSSI_R0: u32 = 0x00000001; // SSI Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRI2C register.
//
// *****************************************************************************
const SYSCTL_PRI2C_R5: u32 = 0x00000020; // I2C Module 5 Peripheral Ready
const SYSCTL_PRI2C_R4: u32 = 0x00000010; // I2C Module 4 Peripheral Ready
const SYSCTL_PRI2C_R3: u32 = 0x00000008; // I2C Module 3 Peripheral Ready
const SYSCTL_PRI2C_R2: u32 = 0x00000004; // I2C Module 2 Peripheral Ready
const SYSCTL_PRI2C_R1: u32 = 0x00000002; // I2C Module 1 Peripheral Ready
const SYSCTL_PRI2C_R0: u32 = 0x00000001; // I2C Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRUSB register.
//
// *****************************************************************************
const SYSCTL_PRUSB_R0: u32 = 0x00000001; // USB Module Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRCAN register.
//
// *****************************************************************************
const SYSCTL_PRCAN_R1: u32 = 0x00000002; // CAN Module 1 Peripheral Ready
const SYSCTL_PRCAN_R0: u32 = 0x00000001; // CAN Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRADC register.
//
// *****************************************************************************
const SYSCTL_PRADC_R1: u32 = 0x00000002; // ADC Module 1 Peripheral Ready
const SYSCTL_PRADC_R0: u32 = 0x00000001; // ADC Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRACMP register.
//
// *****************************************************************************
const SYSCTL_PRACMP_R0: u32 = 0x00000001; // Analog Comparator Module 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PREEPROM
// register.
//
// *****************************************************************************
const SYSCTL_PREEPROM_R0: u32 = 0x00000001; // EEPROM Module Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the SYSCTL_PRWTIMER
// register.
//
// *****************************************************************************
const SYSCTL_PRWTIMER_R5: u32 = 0x00000020; // Wide Timer 5 Peripheral Ready
const SYSCTL_PRWTIMER_R4: u32 = 0x00000010; // Wide Timer 4 Peripheral Ready
const SYSCTL_PRWTIMER_R3: u32 = 0x00000008; // Wide Timer 3 Peripheral Ready
const SYSCTL_PRWTIMER_R2: u32 = 0x00000004; // Wide Timer 2 Peripheral Ready
const SYSCTL_PRWTIMER_R1: u32 = 0x00000002; // Wide Timer 1 Peripheral Ready
const SYSCTL_PRWTIMER_R0: u32 = 0x00000001; // Wide Timer 0 Peripheral Ready

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_STAT register.
//
// *****************************************************************************
const UDMA_STAT_DMACHANS_M: u32 = 0x001F0000; // Available uDMA Channels Minus 1
const UDMA_STAT_STATE_M: u32 = 0x000000F0; // Control State Machine Status
const UDMA_STAT_STATE_IDLE: u32 = 0x00000000; // Idle
const UDMA_STAT_STATE_RD_CTRL: u32 = 0x00000010; // Reading channel controller data
const UDMA_STAT_STATE_RD_SRCENDP: u32 = 0x00000020; // Reading source end pointer
const UDMA_STAT_STATE_RD_DSTENDP: u32 = 0x00000030; // Reading destination end pointer
const UDMA_STAT_STATE_RD_SRCDAT: u32 = 0x00000040; // Reading source data
const UDMA_STAT_STATE_WR_DSTDAT: u32 = 0x00000050; // Writing destination data
const UDMA_STAT_STATE_WAIT: u32 = 0x00000060; // Waiting for uDMA request to clear
const UDMA_STAT_STATE_WR_CTRL: u32 = 0x00000070; // Writing channel controller data
const UDMA_STAT_STATE_STALL: u32 = 0x00000080; // Stalled
const UDMA_STAT_STATE_DONE: u32 = 0x00000090; // Done
const UDMA_STAT_STATE_UNDEF: u32 = 0x000000A0; // Undefined
const UDMA_STAT_MASTEN: u32 = 0x00000001; // Master Enable Status
const UDMA_STAT_DMACHANS_S: u32 = 16;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CFG register.
//
// *****************************************************************************
const UDMA_CFG_MASTEN: u32 = 0x00000001; // Controller Master Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CTLBASE register.
//
// *****************************************************************************
const UDMA_CTLBASE_ADDR_M: u32 = 0xFFFFFC00; // Channel Control Base Address
const UDMA_CTLBASE_ADDR_S: u32 = 10;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ALTBASE register.
//
// *****************************************************************************
const UDMA_ALTBASE_ADDR_M: u32 = 0xFFFFFFFF; // Alternate Channel Address Pointer
const UDMA_ALTBASE_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_WAITSTAT register.
//
// *****************************************************************************
const UDMA_WAITSTAT_WAITREQ_M: u32 = 0xFFFFFFFF; // Channel [n] Wait Status

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_SWREQ register.
//
// *****************************************************************************
const UDMA_SWREQ_M: u32 = 0xFFFFFFFF; // Channel [n] Software Request

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_USEBURSTSET
// register.
//
// *****************************************************************************
const UDMA_USEBURSTSET_SET_M: u32 = 0xFFFFFFFF; // Channel [n] Useburst Set

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_USEBURSTCLR
// register.
//
// *****************************************************************************
const UDMA_USEBURSTCLR_CLR_M: u32 = 0xFFFFFFFF; // Channel [n] Useburst Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_REQMASKSET
// register.
//
// *****************************************************************************
const UDMA_REQMASKSET_SET_M: u32 = 0xFFFFFFFF; // Channel [n] Request Mask Set

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_REQMASKCLR
// register.
//
// *****************************************************************************
const UDMA_REQMASKCLR_CLR_M: u32 = 0xFFFFFFFF; // Channel [n] Request Mask Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ENASET register.
//
// *****************************************************************************
const UDMA_ENASET_SET_M: u32 = 0xFFFFFFFF; // Channel [n] Enable Set

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ENACLR register.
//
// *****************************************************************************
const UDMA_ENACLR_CLR_M: u32 = 0xFFFFFFFF; // Clear Channel [n] Enable Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ALTSET register.
//
// *****************************************************************************
const UDMA_ALTSET_SET_M: u32 = 0xFFFFFFFF; // Channel [n] Alternate Set

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ALTCLR register.
//
// *****************************************************************************
const UDMA_ALTCLR_CLR_M: u32 = 0xFFFFFFFF; // Channel [n] Alternate Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_PRIOSET register.
//
// *****************************************************************************
const UDMA_PRIOSET_SET_M: u32 = 0xFFFFFFFF; // Channel [n] Priority Set

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_PRIOCLR register.
//
// *****************************************************************************
const UDMA_PRIOCLR_CLR_M: u32 = 0xFFFFFFFF; // Channel [n] Priority Clear

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_ERRCLR register.
//
// *****************************************************************************
const UDMA_ERRCLR_ERRCLR: u32 = 0x00000001; // uDMA Bus Error Status

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHASGN register.
//
// *****************************************************************************
const UDMA_CHASGN_M: u32 = 0xFFFFFFFF; // Channel [n] Assignment Select
const UDMA_CHASGN_PRIMARY: u32 = 0x00000000; // Use the primary channel assignment
const UDMA_CHASGN_SECONDARY: u32 = 0x00000001; // Use the secondary channel assignment

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHIS register.
//
// *****************************************************************************
const UDMA_CHIS_M: u32 = 0xFFFFFFFF; // Channel [n] Interrupt Status

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHMAP0 register.
//
// *****************************************************************************
const UDMA_CHMAP0_CH7SEL_M: u32 = 0xF0000000; // uDMA Channel 7 Source Select
const UDMA_CHMAP0_CH6SEL_M: u32 = 0x0F000000; // uDMA Channel 6 Source Select
const UDMA_CHMAP0_CH5SEL_M: u32 = 0x00F00000; // uDMA Channel 5 Source Select
const UDMA_CHMAP0_CH4SEL_M: u32 = 0x000F0000; // uDMA Channel 4 Source Select
const UDMA_CHMAP0_CH3SEL_M: u32 = 0x0000F000; // uDMA Channel 3 Source Select
const UDMA_CHMAP0_CH2SEL_M: u32 = 0x00000F00; // uDMA Channel 2 Source Select
const UDMA_CHMAP0_CH1SEL_M: u32 = 0x000000F0; // uDMA Channel 1 Source Select
const UDMA_CHMAP0_CH0SEL_M: u32 = 0x0000000F; // uDMA Channel 0 Source Select
const UDMA_CHMAP0_CH7SEL_S: u32 = 28;
const UDMA_CHMAP0_CH6SEL_S: u32 = 24;
const UDMA_CHMAP0_CH5SEL_S: u32 = 20;
const UDMA_CHMAP0_CH4SEL_S: u32 = 16;
const UDMA_CHMAP0_CH3SEL_S: u32 = 12;
const UDMA_CHMAP0_CH2SEL_S: u32 = 8;
const UDMA_CHMAP0_CH1SEL_S: u32 = 4;
const UDMA_CHMAP0_CH0SEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHMAP1 register.
//
// *****************************************************************************
const UDMA_CHMAP1_CH15SEL_M: u32 = 0xF0000000; // uDMA Channel 15 Source Select
const UDMA_CHMAP1_CH14SEL_M: u32 = 0x0F000000; // uDMA Channel 14 Source Select
const UDMA_CHMAP1_CH13SEL_M: u32 = 0x00F00000; // uDMA Channel 13 Source Select
const UDMA_CHMAP1_CH12SEL_M: u32 = 0x000F0000; // uDMA Channel 12 Source Select
const UDMA_CHMAP1_CH11SEL_M: u32 = 0x0000F000; // uDMA Channel 11 Source Select
const UDMA_CHMAP1_CH10SEL_M: u32 = 0x00000F00; // uDMA Channel 10 Source Select
const UDMA_CHMAP1_CH9SEL_M: u32 = 0x000000F0; // uDMA Channel 9 Source Select
const UDMA_CHMAP1_CH8SEL_M: u32 = 0x0000000F; // uDMA Channel 8 Source Select
const UDMA_CHMAP1_CH15SEL_S: u32 = 28;
const UDMA_CHMAP1_CH14SEL_S: u32 = 24;
const UDMA_CHMAP1_CH13SEL_S: u32 = 20;
const UDMA_CHMAP1_CH12SEL_S: u32 = 16;
const UDMA_CHMAP1_CH11SEL_S: u32 = 12;
const UDMA_CHMAP1_CH10SEL_S: u32 = 8;
const UDMA_CHMAP1_CH9SEL_S: u32 = 4;
const UDMA_CHMAP1_CH8SEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHMAP2 register.
//
// *****************************************************************************
const UDMA_CHMAP2_CH23SEL_M: u32 = 0xF0000000; // uDMA Channel 23 Source Select
const UDMA_CHMAP2_CH22SEL_M: u32 = 0x0F000000; // uDMA Channel 22 Source Select
const UDMA_CHMAP2_CH21SEL_M: u32 = 0x00F00000; // uDMA Channel 21 Source Select
const UDMA_CHMAP2_CH20SEL_M: u32 = 0x000F0000; // uDMA Channel 20 Source Select
const UDMA_CHMAP2_CH19SEL_M: u32 = 0x0000F000; // uDMA Channel 19 Source Select
const UDMA_CHMAP2_CH18SEL_M: u32 = 0x00000F00; // uDMA Channel 18 Source Select
const UDMA_CHMAP2_CH17SEL_M: u32 = 0x000000F0; // uDMA Channel 17 Source Select
const UDMA_CHMAP2_CH16SEL_M: u32 = 0x0000000F; // uDMA Channel 16 Source Select
const UDMA_CHMAP2_CH23SEL_S: u32 = 28;
const UDMA_CHMAP2_CH22SEL_S: u32 = 24;
const UDMA_CHMAP2_CH21SEL_S: u32 = 20;
const UDMA_CHMAP2_CH20SEL_S: u32 = 16;
const UDMA_CHMAP2_CH19SEL_S: u32 = 12;
const UDMA_CHMAP2_CH18SEL_S: u32 = 8;
const UDMA_CHMAP2_CH17SEL_S: u32 = 4;
const UDMA_CHMAP2_CH16SEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_CHMAP3 register.
//
// *****************************************************************************
const UDMA_CHMAP3_CH31SEL_M: u32 = 0xF0000000; // uDMA Channel 31 Source Select
const UDMA_CHMAP3_CH30SEL_M: u32 = 0x0F000000; // uDMA Channel 30 Source Select
const UDMA_CHMAP3_CH29SEL_M: u32 = 0x00F00000; // uDMA Channel 29 Source Select
const UDMA_CHMAP3_CH28SEL_M: u32 = 0x000F0000; // uDMA Channel 28 Source Select
const UDMA_CHMAP3_CH27SEL_M: u32 = 0x0000F000; // uDMA Channel 27 Source Select
const UDMA_CHMAP3_CH26SEL_M: u32 = 0x00000F00; // uDMA Channel 26 Source Select
const UDMA_CHMAP3_CH25SEL_M: u32 = 0x000000F0; // uDMA Channel 25 Source Select
const UDMA_CHMAP3_CH24SEL_M: u32 = 0x0000000F; // uDMA Channel 24 Source Select
const UDMA_CHMAP3_CH31SEL_S: u32 = 28;
const UDMA_CHMAP3_CH30SEL_S: u32 = 24;
const UDMA_CHMAP3_CH29SEL_S: u32 = 20;
const UDMA_CHMAP3_CH28SEL_S: u32 = 16;
const UDMA_CHMAP3_CH27SEL_S: u32 = 12;
const UDMA_CHMAP3_CH26SEL_S: u32 = 8;
const UDMA_CHMAP3_CH25SEL_S: u32 = 4;
const UDMA_CHMAP3_CH24SEL_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_O_SRCENDP register.
//
// *****************************************************************************
const UDMA_SRCENDP_ADDR_M: u32 = 0xFFFFFFFF; // Source Address End Pointer
const UDMA_SRCENDP_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_O_DSTENDP register.
//
// *****************************************************************************
const UDMA_DSTENDP_ADDR_M: u32 = 0xFFFFFFFF; // Destination Address End Pointer
const UDMA_DSTENDP_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the UDMA_O_CHCTL register.
//
// *****************************************************************************
const UDMA_CHCTL_DSTINC_M: u32 = 0xC0000000; // Destination Address Increment
const UDMA_CHCTL_DSTINC_8: u32 = 0x00000000; // Byte
const UDMA_CHCTL_DSTINC_16: u32 = 0x40000000; // Half-word
const UDMA_CHCTL_DSTINC_32: u32 = 0x80000000; // Word
const UDMA_CHCTL_DSTINC_NONE: u32 = 0xC0000000; // No increment
const UDMA_CHCTL_DSTSIZE_M: u32 = 0x30000000; // Destination Data Size
const UDMA_CHCTL_DSTSIZE_8: u32 = 0x00000000; // Byte
const UDMA_CHCTL_DSTSIZE_16: u32 = 0x10000000; // Half-word
const UDMA_CHCTL_DSTSIZE_32: u32 = 0x20000000; // Word
const UDMA_CHCTL_SRCINC_M: u32 = 0x0C000000; // Source Address Increment
const UDMA_CHCTL_SRCINC_8: u32 = 0x00000000; // Byte
const UDMA_CHCTL_SRCINC_16: u32 = 0x04000000; // Half-word
const UDMA_CHCTL_SRCINC_32: u32 = 0x08000000; // Word
const UDMA_CHCTL_SRCINC_NONE: u32 = 0x0C000000; // No increment
const UDMA_CHCTL_SRCSIZE_M: u32 = 0x03000000; // Source Data Size
const UDMA_CHCTL_SRCSIZE_8: u32 = 0x00000000; // Byte
const UDMA_CHCTL_SRCSIZE_16: u32 = 0x01000000; // Half-word
const UDMA_CHCTL_SRCSIZE_32: u32 = 0x02000000; // Word
const UDMA_CHCTL_ARBSIZE_M: u32 = 0x0003C000; // Arbitration Size
const UDMA_CHCTL_ARBSIZE_1: u32 = 0x00000000; // 1 Transfer
const UDMA_CHCTL_ARBSIZE_2: u32 = 0x00004000; // 2 Transfers
const UDMA_CHCTL_ARBSIZE_4: u32 = 0x00008000; // 4 Transfers
const UDMA_CHCTL_ARBSIZE_8: u32 = 0x0000C000; // 8 Transfers
const UDMA_CHCTL_ARBSIZE_16: u32 = 0x00010000; // 16 Transfers
const UDMA_CHCTL_ARBSIZE_32: u32 = 0x00014000; // 32 Transfers
const UDMA_CHCTL_ARBSIZE_64: u32 = 0x00018000; // 64 Transfers
const UDMA_CHCTL_ARBSIZE_128: u32 = 0x0001C000; // 128 Transfers
const UDMA_CHCTL_ARBSIZE_256: u32 = 0x00020000; // 256 Transfers
const UDMA_CHCTL_ARBSIZE_512: u32 = 0x00024000; // 512 Transfers
const UDMA_CHCTL_ARBSIZE_1024: u32 = 0x00028000; // 1024 Transfers
const UDMA_CHCTL_XFERSIZE_M: u32 = 0x00003FF0; // Transfer Size (minus 1)
const UDMA_CHCTL_NXTUSEBURST: u32 = 0x00000008; // Next Useburst
const UDMA_CHCTL_XFERMODE_M: u32 = 0x00000007; // uDMA Transfer Mode
const UDMA_CHCTL_XFERMODE_STOP: u32 = 0x00000000; // Stop
const UDMA_CHCTL_XFERMODE_BASIC: u32 = 0x00000001; // Basic
const UDMA_CHCTL_XFERMODE_AUTO: u32 = 0x00000002; // Auto-Request
const UDMA_CHCTL_XFERMODE_PINGPONG: u32 = 0x00000003; // Ping-Pong
const UDMA_CHCTL_XFERMODE_MEM_SG: u32 = 0x00000004; // Memory Scatter-Gather
const UDMA_CHCTL_XFERMODE_MEM_SGA: u32 = 0x00000005; // Alternate Memory Scatter-Gather
const UDMA_CHCTL_XFERMODE_PER_SG: u32 = 0x00000006; // Peripheral Scatter-Gather
const UDMA_CHCTL_XFERMODE_PER_SGA: u32 = 0x00000007; // Alternate Peripheral Scatter-Gather
const UDMA_CHCTL_XFERSIZE_S: u32 = 4;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_INT_TYPE register.
//
// *****************************************************************************
const NVIC_INT_TYPE_LINES_M: u32 = 0x0000001F; // Number of interrupt lines (x32)
const NVIC_INT_TYPE_LINES_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTLR register.
//
// *****************************************************************************
const NVIC_ACTLR_DISOOFP: u32 = 0x00000200; // Disable Out-Of-Order Floating Point
const NVIC_ACTLR_DISFPCA: u32 = 0x00000100; // Disable CONTROL
const NVIC_ACTLR_DISFOLD: u32 = 0x00000004; // Disable IT Folding
const NVIC_ACTLR_DISWBUF: u32 = 0x00000002; // Disable Write Buffer
const NVIC_ACTLR_DISMCYC: u32 = 0x00000001; // Disable Interrupts of Multiple Cycle Instructions

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ST_CTRL register.
//
// *****************************************************************************
const NVIC_ST_CTRL_COUNT: u32 = 0x00010000; // Count Flag
const NVIC_ST_CTRL_CLK_SRC: u32 = 0x00000004; // Clock Source
const NVIC_ST_CTRL_INTEN: u32 = 0x00000002; // Interrupt Enable
const NVIC_ST_CTRL_ENABLE: u32 = 0x00000001; // Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ST_RELOAD register.
//
// *****************************************************************************
const NVIC_ST_RELOAD_M: u32 = 0x00FFFFFF; // Reload Value
const NVIC_ST_RELOAD_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ST_CURRENT
// register.
//
// *****************************************************************************
const NVIC_ST_CURRENT_M: u32 = 0x00FFFFFF; // Current Value
const NVIC_ST_CURRENT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ST_CAL register.
//
// *****************************************************************************
const NVIC_ST_CAL_NOREF: u32 = 0x80000000; // No reference clock
const NVIC_ST_CAL_SKEW: u32 = 0x40000000; // Clock skew
const NVIC_ST_CAL_ONEMS_M: u32 = 0x00FFFFFF; // 1ms reference value
const NVIC_ST_CAL_ONEMS_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_EN0 register.
//
// *****************************************************************************
const NVIC_EN0_INT_M: u32 = 0xFFFFFFFF; // Interrupt Enable
const NVIC_EN0_INT0: u32 = 0x00000001; // Interrupt 0 enable
const NVIC_EN0_INT1: u32 = 0x00000002; // Interrupt 1 enable
const NVIC_EN0_INT2: u32 = 0x00000004; // Interrupt 2 enable
const NVIC_EN0_INT3: u32 = 0x00000008; // Interrupt 3 enable
const NVIC_EN0_INT4: u32 = 0x00000010; // Interrupt 4 enable
const NVIC_EN0_INT5: u32 = 0x00000020; // Interrupt 5 enable
const NVIC_EN0_INT6: u32 = 0x00000040; // Interrupt 6 enable
const NVIC_EN0_INT7: u32 = 0x00000080; // Interrupt 7 enable
const NVIC_EN0_INT8: u32 = 0x00000100; // Interrupt 8 enable
const NVIC_EN0_INT9: u32 = 0x00000200; // Interrupt 9 enable
const NVIC_EN0_INT10: u32 = 0x00000400; // Interrupt 10 enable
const NVIC_EN0_INT11: u32 = 0x00000800; // Interrupt 11 enable
const NVIC_EN0_INT12: u32 = 0x00001000; // Interrupt 12 enable
const NVIC_EN0_INT13: u32 = 0x00002000; // Interrupt 13 enable
const NVIC_EN0_INT14: u32 = 0x00004000; // Interrupt 14 enable
const NVIC_EN0_INT15: u32 = 0x00008000; // Interrupt 15 enable
const NVIC_EN0_INT16: u32 = 0x00010000; // Interrupt 16 enable
const NVIC_EN0_INT17: u32 = 0x00020000; // Interrupt 17 enable
const NVIC_EN0_INT18: u32 = 0x00040000; // Interrupt 18 enable
const NVIC_EN0_INT19: u32 = 0x00080000; // Interrupt 19 enable
const NVIC_EN0_INT20: u32 = 0x00100000; // Interrupt 20 enable
const NVIC_EN0_INT21: u32 = 0x00200000; // Interrupt 21 enable
const NVIC_EN0_INT22: u32 = 0x00400000; // Interrupt 22 enable
const NVIC_EN0_INT23: u32 = 0x00800000; // Interrupt 23 enable
const NVIC_EN0_INT24: u32 = 0x01000000; // Interrupt 24 enable
const NVIC_EN0_INT25: u32 = 0x02000000; // Interrupt 25 enable
const NVIC_EN0_INT26: u32 = 0x04000000; // Interrupt 26 enable
const NVIC_EN0_INT27: u32 = 0x08000000; // Interrupt 27 enable
const NVIC_EN0_INT28: u32 = 0x10000000; // Interrupt 28 enable
const NVIC_EN0_INT29: u32 = 0x20000000; // Interrupt 29 enable
const NVIC_EN0_INT30: u32 = 0x40000000; // Interrupt 30 enable
const NVIC_EN0_INT31: u32 = 0x80000000; // Interrupt 31 enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_EN1 register.
//
// *****************************************************************************
const NVIC_EN1_INT_M: u32 = 0xFFFFFFFF; // Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_EN2 register.
//
// *****************************************************************************
const NVIC_EN2_INT_M: u32 = 0xFFFFFFFF; // Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_EN3 register.
//
// *****************************************************************************
const NVIC_EN3_INT_M: u32 = 0xFFFFFFFF; // Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_EN4 register.
//
// *****************************************************************************
const NVIC_EN4_INT_M: u32 = 0x000007FF; // Interrupt Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DIS0 register.
//
// *****************************************************************************
const NVIC_DIS0_INT_M: u32 = 0xFFFFFFFF; // Interrupt Disable
const NVIC_DIS0_INT0: u32 = 0x00000001; // Interrupt 0 disable
const NVIC_DIS0_INT1: u32 = 0x00000002; // Interrupt 1 disable
const NVIC_DIS0_INT2: u32 = 0x00000004; // Interrupt 2 disable
const NVIC_DIS0_INT3: u32 = 0x00000008; // Interrupt 3 disable
const NVIC_DIS0_INT4: u32 = 0x00000010; // Interrupt 4 disable
const NVIC_DIS0_INT5: u32 = 0x00000020; // Interrupt 5 disable
const NVIC_DIS0_INT6: u32 = 0x00000040; // Interrupt 6 disable
const NVIC_DIS0_INT7: u32 = 0x00000080; // Interrupt 7 disable
const NVIC_DIS0_INT8: u32 = 0x00000100; // Interrupt 8 disable
const NVIC_DIS0_INT9: u32 = 0x00000200; // Interrupt 9 disable
const NVIC_DIS0_INT10: u32 = 0x00000400; // Interrupt 10 disable
const NVIC_DIS0_INT11: u32 = 0x00000800; // Interrupt 11 disable
const NVIC_DIS0_INT12: u32 = 0x00001000; // Interrupt 12 disable
const NVIC_DIS0_INT13: u32 = 0x00002000; // Interrupt 13 disable
const NVIC_DIS0_INT14: u32 = 0x00004000; // Interrupt 14 disable
const NVIC_DIS0_INT15: u32 = 0x00008000; // Interrupt 15 disable
const NVIC_DIS0_INT16: u32 = 0x00010000; // Interrupt 16 disable
const NVIC_DIS0_INT17: u32 = 0x00020000; // Interrupt 17 disable
const NVIC_DIS0_INT18: u32 = 0x00040000; // Interrupt 18 disable
const NVIC_DIS0_INT19: u32 = 0x00080000; // Interrupt 19 disable
const NVIC_DIS0_INT20: u32 = 0x00100000; // Interrupt 20 disable
const NVIC_DIS0_INT21: u32 = 0x00200000; // Interrupt 21 disable
const NVIC_DIS0_INT22: u32 = 0x00400000; // Interrupt 22 disable
const NVIC_DIS0_INT23: u32 = 0x00800000; // Interrupt 23 disable
const NVIC_DIS0_INT24: u32 = 0x01000000; // Interrupt 24 disable
const NVIC_DIS0_INT25: u32 = 0x02000000; // Interrupt 25 disable
const NVIC_DIS0_INT26: u32 = 0x04000000; // Interrupt 26 disable
const NVIC_DIS0_INT27: u32 = 0x08000000; // Interrupt 27 disable
const NVIC_DIS0_INT28: u32 = 0x10000000; // Interrupt 28 disable
const NVIC_DIS0_INT29: u32 = 0x20000000; // Interrupt 29 disable
const NVIC_DIS0_INT30: u32 = 0x40000000; // Interrupt 30 disable
const NVIC_DIS0_INT31: u32 = 0x80000000; // Interrupt 31 disable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DIS1 register.
//
// *****************************************************************************
const NVIC_DIS1_INT_M: u32 = 0xFFFFFFFF; // Interrupt Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DIS2 register.
//
// *****************************************************************************
const NVIC_DIS2_INT_M: u32 = 0xFFFFFFFF; // Interrupt Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DIS3 register.
//
// *****************************************************************************
const NVIC_DIS3_INT_M: u32 = 0xFFFFFFFF; // Interrupt Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DIS4 register.
//
// *****************************************************************************
const NVIC_DIS4_INT_M: u32 = 0x000007FF; // Interrupt Disable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PEND0 register.
//
// *****************************************************************************
const NVIC_PEND0_INT_M: u32 = 0xFFFFFFFF; // Interrupt Set Pending
const NVIC_PEND0_INT0: u32 = 0x00000001; // Interrupt 0 pend
const NVIC_PEND0_INT1: u32 = 0x00000002; // Interrupt 1 pend
const NVIC_PEND0_INT2: u32 = 0x00000004; // Interrupt 2 pend
const NVIC_PEND0_INT3: u32 = 0x00000008; // Interrupt 3 pend
const NVIC_PEND0_INT4: u32 = 0x00000010; // Interrupt 4 pend
const NVIC_PEND0_INT5: u32 = 0x00000020; // Interrupt 5 pend
const NVIC_PEND0_INT6: u32 = 0x00000040; // Interrupt 6 pend
const NVIC_PEND0_INT7: u32 = 0x00000080; // Interrupt 7 pend
const NVIC_PEND0_INT8: u32 = 0x00000100; // Interrupt 8 pend
const NVIC_PEND0_INT9: u32 = 0x00000200; // Interrupt 9 pend
const NVIC_PEND0_INT10: u32 = 0x00000400; // Interrupt 10 pend
const NVIC_PEND0_INT11: u32 = 0x00000800; // Interrupt 11 pend
const NVIC_PEND0_INT12: u32 = 0x00001000; // Interrupt 12 pend
const NVIC_PEND0_INT13: u32 = 0x00002000; // Interrupt 13 pend
const NVIC_PEND0_INT14: u32 = 0x00004000; // Interrupt 14 pend
const NVIC_PEND0_INT15: u32 = 0x00008000; // Interrupt 15 pend
const NVIC_PEND0_INT16: u32 = 0x00010000; // Interrupt 16 pend
const NVIC_PEND0_INT17: u32 = 0x00020000; // Interrupt 17 pend
const NVIC_PEND0_INT18: u32 = 0x00040000; // Interrupt 18 pend
const NVIC_PEND0_INT19: u32 = 0x00080000; // Interrupt 19 pend
const NVIC_PEND0_INT20: u32 = 0x00100000; // Interrupt 20 pend
const NVIC_PEND0_INT21: u32 = 0x00200000; // Interrupt 21 pend
const NVIC_PEND0_INT22: u32 = 0x00400000; // Interrupt 22 pend
const NVIC_PEND0_INT23: u32 = 0x00800000; // Interrupt 23 pend
const NVIC_PEND0_INT24: u32 = 0x01000000; // Interrupt 24 pend
const NVIC_PEND0_INT25: u32 = 0x02000000; // Interrupt 25 pend
const NVIC_PEND0_INT26: u32 = 0x04000000; // Interrupt 26 pend
const NVIC_PEND0_INT27: u32 = 0x08000000; // Interrupt 27 pend
const NVIC_PEND0_INT28: u32 = 0x10000000; // Interrupt 28 pend
const NVIC_PEND0_INT29: u32 = 0x20000000; // Interrupt 29 pend
const NVIC_PEND0_INT30: u32 = 0x40000000; // Interrupt 30 pend
const NVIC_PEND0_INT31: u32 = 0x80000000; // Interrupt 31 pend

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PEND1 register.
//
// *****************************************************************************
const NVIC_PEND1_INT_M: u32 = 0xFFFFFFFF; // Interrupt Set Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PEND2 register.
//
// *****************************************************************************
const NVIC_PEND2_INT_M: u32 = 0xFFFFFFFF; // Interrupt Set Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PEND3 register.
//
// *****************************************************************************
const NVIC_PEND3_INT_M: u32 = 0xFFFFFFFF; // Interrupt Set Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PEND4 register.
//
// *****************************************************************************
const NVIC_PEND4_INT_M: u32 = 0x000007FF; // Interrupt Set Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_UNPEND0 register.
//
// *****************************************************************************
const NVIC_UNPEND0_INT_M: u32 = 0xFFFFFFFF; // Interrupt Clear Pending
const NVIC_UNPEND0_INT0: u32 = 0x00000001; // Interrupt 0 unpend
const NVIC_UNPEND0_INT1: u32 = 0x00000002; // Interrupt 1 unpend
const NVIC_UNPEND0_INT2: u32 = 0x00000004; // Interrupt 2 unpend
const NVIC_UNPEND0_INT3: u32 = 0x00000008; // Interrupt 3 unpend
const NVIC_UNPEND0_INT4: u32 = 0x00000010; // Interrupt 4 unpend
const NVIC_UNPEND0_INT5: u32 = 0x00000020; // Interrupt 5 unpend
const NVIC_UNPEND0_INT6: u32 = 0x00000040; // Interrupt 6 unpend
const NVIC_UNPEND0_INT7: u32 = 0x00000080; // Interrupt 7 unpend
const NVIC_UNPEND0_INT8: u32 = 0x00000100; // Interrupt 8 unpend
const NVIC_UNPEND0_INT9: u32 = 0x00000200; // Interrupt 9 unpend
const NVIC_UNPEND0_INT10: u32 = 0x00000400; // Interrupt 10 unpend
const NVIC_UNPEND0_INT11: u32 = 0x00000800; // Interrupt 11 unpend
const NVIC_UNPEND0_INT12: u32 = 0x00001000; // Interrupt 12 unpend
const NVIC_UNPEND0_INT13: u32 = 0x00002000; // Interrupt 13 unpend
const NVIC_UNPEND0_INT14: u32 = 0x00004000; // Interrupt 14 unpend
const NVIC_UNPEND0_INT15: u32 = 0x00008000; // Interrupt 15 unpend
const NVIC_UNPEND0_INT16: u32 = 0x00010000; // Interrupt 16 unpend
const NVIC_UNPEND0_INT17: u32 = 0x00020000; // Interrupt 17 unpend
const NVIC_UNPEND0_INT18: u32 = 0x00040000; // Interrupt 18 unpend
const NVIC_UNPEND0_INT19: u32 = 0x00080000; // Interrupt 19 unpend
const NVIC_UNPEND0_INT20: u32 = 0x00100000; // Interrupt 20 unpend
const NVIC_UNPEND0_INT21: u32 = 0x00200000; // Interrupt 21 unpend
const NVIC_UNPEND0_INT22: u32 = 0x00400000; // Interrupt 22 unpend
const NVIC_UNPEND0_INT23: u32 = 0x00800000; // Interrupt 23 unpend
const NVIC_UNPEND0_INT24: u32 = 0x01000000; // Interrupt 24 unpend
const NVIC_UNPEND0_INT25: u32 = 0x02000000; // Interrupt 25 unpend
const NVIC_UNPEND0_INT26: u32 = 0x04000000; // Interrupt 26 unpend
const NVIC_UNPEND0_INT27: u32 = 0x08000000; // Interrupt 27 unpend
const NVIC_UNPEND0_INT28: u32 = 0x10000000; // Interrupt 28 unpend
const NVIC_UNPEND0_INT29: u32 = 0x20000000; // Interrupt 29 unpend
const NVIC_UNPEND0_INT30: u32 = 0x40000000; // Interrupt 30 unpend
const NVIC_UNPEND0_INT31: u32 = 0x80000000; // Interrupt 31 unpend

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_UNPEND1 register.
//
// *****************************************************************************
const NVIC_UNPEND1_INT_M: u32 = 0xFFFFFFFF; // Interrupt Clear Pending
const NVIC_UNPEND1_INT32: u32 = 0x00000001; // Interrupt 32 unpend
const NVIC_UNPEND1_INT33: u32 = 0x00000002; // Interrupt 33 unpend
const NVIC_UNPEND1_INT34: u32 = 0x00000004; // Interrupt 34 unpend
const NVIC_UNPEND1_INT35: u32 = 0x00000008; // Interrupt 35 unpend
const NVIC_UNPEND1_INT36: u32 = 0x00000010; // Interrupt 36 unpend
const NVIC_UNPEND1_INT37: u32 = 0x00000020; // Interrupt 37 unpend
const NVIC_UNPEND1_INT38: u32 = 0x00000040; // Interrupt 38 unpend
const NVIC_UNPEND1_INT39: u32 = 0x00000080; // Interrupt 39 unpend
const NVIC_UNPEND1_INT40: u32 = 0x00000100; // Interrupt 40 unpend
const NVIC_UNPEND1_INT41: u32 = 0x00000200; // Interrupt 41 unpend
const NVIC_UNPEND1_INT42: u32 = 0x00000400; // Interrupt 42 unpend
const NVIC_UNPEND1_INT43: u32 = 0x00000800; // Interrupt 43 unpend
const NVIC_UNPEND1_INT44: u32 = 0x00001000; // Interrupt 44 unpend
const NVIC_UNPEND1_INT45: u32 = 0x00002000; // Interrupt 45 unpend
const NVIC_UNPEND1_INT46: u32 = 0x00004000; // Interrupt 46 unpend
const NVIC_UNPEND1_INT47: u32 = 0x00008000; // Interrupt 47 unpend
const NVIC_UNPEND1_INT48: u32 = 0x00010000; // Interrupt 48 unpend
const NVIC_UNPEND1_INT49: u32 = 0x00020000; // Interrupt 49 unpend
const NVIC_UNPEND1_INT50: u32 = 0x00040000; // Interrupt 50 unpend
const NVIC_UNPEND1_INT51: u32 = 0x00080000; // Interrupt 51 unpend
const NVIC_UNPEND1_INT52: u32 = 0x00100000; // Interrupt 52 unpend
const NVIC_UNPEND1_INT53: u32 = 0x00200000; // Interrupt 53 unpend
const NVIC_UNPEND1_INT54: u32 = 0x00400000; // Interrupt 54 unpend
const NVIC_UNPEND1_INT55: u32 = 0x00800000; // Interrupt 55 unpend

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_UNPEND2 register.
//
// *****************************************************************************
const NVIC_UNPEND2_INT_M: u32 = 0xFFFFFFFF; // Interrupt Clear Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_UNPEND3 register.
//
// *****************************************************************************
const NVIC_UNPEND3_INT_M: u32 = 0xFFFFFFFF; // Interrupt Clear Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_UNPEND4 register.
//
// *****************************************************************************
const NVIC_UNPEND4_INT_M: u32 = 0x000007FF; // Interrupt Clear Pending

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTIVE0 register.
//
// *****************************************************************************
const NVIC_ACTIVE0_INT_M: u32 = 0xFFFFFFFF; // Interrupt Active
const NVIC_ACTIVE0_INT0: u32 = 0x00000001; // Interrupt 0 active
const NVIC_ACTIVE0_INT1: u32 = 0x00000002; // Interrupt 1 active
const NVIC_ACTIVE0_INT2: u32 = 0x00000004; // Interrupt 2 active
const NVIC_ACTIVE0_INT3: u32 = 0x00000008; // Interrupt 3 active
const NVIC_ACTIVE0_INT4: u32 = 0x00000010; // Interrupt 4 active
const NVIC_ACTIVE0_INT5: u32 = 0x00000020; // Interrupt 5 active
const NVIC_ACTIVE0_INT6: u32 = 0x00000040; // Interrupt 6 active
const NVIC_ACTIVE0_INT7: u32 = 0x00000080; // Interrupt 7 active
const NVIC_ACTIVE0_INT8: u32 = 0x00000100; // Interrupt 8 active
const NVIC_ACTIVE0_INT9: u32 = 0x00000200; // Interrupt 9 active
const NVIC_ACTIVE0_INT10: u32 = 0x00000400; // Interrupt 10 active
const NVIC_ACTIVE0_INT11: u32 = 0x00000800; // Interrupt 11 active
const NVIC_ACTIVE0_INT12: u32 = 0x00001000; // Interrupt 12 active
const NVIC_ACTIVE0_INT13: u32 = 0x00002000; // Interrupt 13 active
const NVIC_ACTIVE0_INT14: u32 = 0x00004000; // Interrupt 14 active
const NVIC_ACTIVE0_INT15: u32 = 0x00008000; // Interrupt 15 active
const NVIC_ACTIVE0_INT16: u32 = 0x00010000; // Interrupt 16 active
const NVIC_ACTIVE0_INT17: u32 = 0x00020000; // Interrupt 17 active
const NVIC_ACTIVE0_INT18: u32 = 0x00040000; // Interrupt 18 active
const NVIC_ACTIVE0_INT19: u32 = 0x00080000; // Interrupt 19 active
const NVIC_ACTIVE0_INT20: u32 = 0x00100000; // Interrupt 20 active
const NVIC_ACTIVE0_INT21: u32 = 0x00200000; // Interrupt 21 active
const NVIC_ACTIVE0_INT22: u32 = 0x00400000; // Interrupt 22 active
const NVIC_ACTIVE0_INT23: u32 = 0x00800000; // Interrupt 23 active
const NVIC_ACTIVE0_INT24: u32 = 0x01000000; // Interrupt 24 active
const NVIC_ACTIVE0_INT25: u32 = 0x02000000; // Interrupt 25 active
const NVIC_ACTIVE0_INT26: u32 = 0x04000000; // Interrupt 26 active
const NVIC_ACTIVE0_INT27: u32 = 0x08000000; // Interrupt 27 active
const NVIC_ACTIVE0_INT28: u32 = 0x10000000; // Interrupt 28 active
const NVIC_ACTIVE0_INT29: u32 = 0x20000000; // Interrupt 29 active
const NVIC_ACTIVE0_INT30: u32 = 0x40000000; // Interrupt 30 active
const NVIC_ACTIVE0_INT31: u32 = 0x80000000; // Interrupt 31 active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTIVE1 register.
//
// *****************************************************************************
const NVIC_ACTIVE1_INT_M: u32 = 0xFFFFFFFF; // Interrupt Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTIVE2 register.
//
// *****************************************************************************
const NVIC_ACTIVE2_INT_M: u32 = 0xFFFFFFFF; // Interrupt Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTIVE3 register.
//
// *****************************************************************************
const NVIC_ACTIVE3_INT_M: u32 = 0xFFFFFFFF; // Interrupt Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_ACTIVE4 register.
//
// *****************************************************************************
const NVIC_ACTIVE4_INT_M: u32 = 0x000007FF; // Interrupt Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI0 register.
//
// *****************************************************************************
const NVIC_PRI0_INT3_M: u32 = 0xE0000000; // Interrupt 3 Priority Mask
const NVIC_PRI0_INT2_M: u32 = 0x00E00000; // Interrupt 2 Priority Mask
const NVIC_PRI0_INT1_M: u32 = 0x0000E000; // Interrupt 1 Priority Mask
const NVIC_PRI0_INT0_M: u32 = 0x000000E0; // Interrupt 0 Priority Mask
const NVIC_PRI0_INT3_S: u32 = 29;
const NVIC_PRI0_INT2_S: u32 = 21;
const NVIC_PRI0_INT1_S: u32 = 13;
const NVIC_PRI0_INT0_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI1 register.
//
// *****************************************************************************
const NVIC_PRI1_INT7_M: u32 = 0xE0000000; // Interrupt 7 Priority Mask
const NVIC_PRI1_INT6_M: u32 = 0x00E00000; // Interrupt 6 Priority Mask
const NVIC_PRI1_INT5_M: u32 = 0x0000E000; // Interrupt 5 Priority Mask
const NVIC_PRI1_INT4_M: u32 = 0x000000E0; // Interrupt 4 Priority Mask
const NVIC_PRI1_INT7_S: u32 = 29;
const NVIC_PRI1_INT6_S: u32 = 21;
const NVIC_PRI1_INT5_S: u32 = 13;
const NVIC_PRI1_INT4_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI2 register.
//
// *****************************************************************************
const NVIC_PRI2_INT11_M: u32 = 0xE0000000; // Interrupt 11 Priority Mask
const NVIC_PRI2_INT10_M: u32 = 0x00E00000; // Interrupt 10 Priority Mask
const NVIC_PRI2_INT9_M: u32 = 0x0000E000; // Interrupt 9 Priority Mask
const NVIC_PRI2_INT8_M: u32 = 0x000000E0; // Interrupt 8 Priority Mask
const NVIC_PRI2_INT11_S: u32 = 29;
const NVIC_PRI2_INT10_S: u32 = 21;
const NVIC_PRI2_INT9_S: u32 = 13;
const NVIC_PRI2_INT8_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI3 register.
//
// *****************************************************************************
const NVIC_PRI3_INT15_M: u32 = 0xE0000000; // Interrupt 15 Priority Mask
const NVIC_PRI3_INT14_M: u32 = 0x00E00000; // Interrupt 14 Priority Mask
const NVIC_PRI3_INT13_M: u32 = 0x0000E000; // Interrupt 13 Priority Mask
const NVIC_PRI3_INT12_M: u32 = 0x000000E0; // Interrupt 12 Priority Mask
const NVIC_PRI3_INT15_S: u32 = 29;
const NVIC_PRI3_INT14_S: u32 = 21;
const NVIC_PRI3_INT13_S: u32 = 13;
const NVIC_PRI3_INT12_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI4 register.
//
// *****************************************************************************
const NVIC_PRI4_INT19_M: u32 = 0xE0000000; // Interrupt 19 Priority Mask
const NVIC_PRI4_INT18_M: u32 = 0x00E00000; // Interrupt 18 Priority Mask
const NVIC_PRI4_INT17_M: u32 = 0x0000E000; // Interrupt 17 Priority Mask
const NVIC_PRI4_INT16_M: u32 = 0x000000E0; // Interrupt 16 Priority Mask
const NVIC_PRI4_INT19_S: u32 = 29;
const NVIC_PRI4_INT18_S: u32 = 21;
const NVIC_PRI4_INT17_S: u32 = 13;
const NVIC_PRI4_INT16_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI5 register.
//
// *****************************************************************************
const NVIC_PRI5_INT23_M: u32 = 0xE0000000; // Interrupt 23 Priority Mask
const NVIC_PRI5_INT22_M: u32 = 0x00E00000; // Interrupt 22 Priority Mask
const NVIC_PRI5_INT21_M: u32 = 0x0000E000; // Interrupt 21 Priority Mask
const NVIC_PRI5_INT20_M: u32 = 0x000000E0; // Interrupt 20 Priority Mask
const NVIC_PRI5_INT23_S: u32 = 29;
const NVIC_PRI5_INT22_S: u32 = 21;
const NVIC_PRI5_INT21_S: u32 = 13;
const NVIC_PRI5_INT20_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI6 register.
//
// *****************************************************************************
const NVIC_PRI6_INT27_M: u32 = 0xE0000000; // Interrupt 27 Priority Mask
const NVIC_PRI6_INT26_M: u32 = 0x00E00000; // Interrupt 26 Priority Mask
const NVIC_PRI6_INT25_M: u32 = 0x0000E000; // Interrupt 25 Priority Mask
const NVIC_PRI6_INT24_M: u32 = 0x000000E0; // Interrupt 24 Priority Mask
const NVIC_PRI6_INT27_S: u32 = 29;
const NVIC_PRI6_INT26_S: u32 = 21;
const NVIC_PRI6_INT25_S: u32 = 13;
const NVIC_PRI6_INT24_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI7 register.
//
// *****************************************************************************
const NVIC_PRI7_INT31_M: u32 = 0xE0000000; // Interrupt 31 Priority Mask
const NVIC_PRI7_INT30_M: u32 = 0x00E00000; // Interrupt 30 Priority Mask
const NVIC_PRI7_INT29_M: u32 = 0x0000E000; // Interrupt 29 Priority Mask
const NVIC_PRI7_INT28_M: u32 = 0x000000E0; // Interrupt 28 Priority Mask
const NVIC_PRI7_INT31_S: u32 = 29;
const NVIC_PRI7_INT30_S: u32 = 21;
const NVIC_PRI7_INT29_S: u32 = 13;
const NVIC_PRI7_INT28_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI8 register.
//
// *****************************************************************************
const NVIC_PRI8_INT35_M: u32 = 0xE0000000; // Interrupt 35 Priority Mask
const NVIC_PRI8_INT34_M: u32 = 0x00E00000; // Interrupt 34 Priority Mask
const NVIC_PRI8_INT33_M: u32 = 0x0000E000; // Interrupt 33 Priority Mask
const NVIC_PRI8_INT32_M: u32 = 0x000000E0; // Interrupt 32 Priority Mask
const NVIC_PRI8_INT35_S: u32 = 29;
const NVIC_PRI8_INT34_S: u32 = 21;
const NVIC_PRI8_INT33_S: u32 = 13;
const NVIC_PRI8_INT32_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI9 register.
//
// *****************************************************************************
const NVIC_PRI9_INT39_M: u32 = 0xE0000000; // Interrupt 39 Priority Mask
const NVIC_PRI9_INT38_M: u32 = 0x00E00000; // Interrupt 38 Priority Mask
const NVIC_PRI9_INT37_M: u32 = 0x0000E000; // Interrupt 37 Priority Mask
const NVIC_PRI9_INT36_M: u32 = 0x000000E0; // Interrupt 36 Priority Mask
const NVIC_PRI9_INT39_S: u32 = 29;
const NVIC_PRI9_INT38_S: u32 = 21;
const NVIC_PRI9_INT37_S: u32 = 13;
const NVIC_PRI9_INT36_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI10 register.
//
// *****************************************************************************
const NVIC_PRI10_INT43_M: u32 = 0xE0000000; // Interrupt 43 Priority Mask
const NVIC_PRI10_INT42_M: u32 = 0x00E00000; // Interrupt 42 Priority Mask
const NVIC_PRI10_INT41_M: u32 = 0x0000E000; // Interrupt 41 Priority Mask
const NVIC_PRI10_INT40_M: u32 = 0x000000E0; // Interrupt 40 Priority Mask
const NVIC_PRI10_INT43_S: u32 = 29;
const NVIC_PRI10_INT42_S: u32 = 21;
const NVIC_PRI10_INT41_S: u32 = 13;
const NVIC_PRI10_INT40_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI11 register.
//
// *****************************************************************************
const NVIC_PRI11_INT47_M: u32 = 0xE0000000; // Interrupt 47 Priority Mask
const NVIC_PRI11_INT46_M: u32 = 0x00E00000; // Interrupt 46 Priority Mask
const NVIC_PRI11_INT45_M: u32 = 0x0000E000; // Interrupt 45 Priority Mask
const NVIC_PRI11_INT44_M: u32 = 0x000000E0; // Interrupt 44 Priority Mask
const NVIC_PRI11_INT47_S: u32 = 29;
const NVIC_PRI11_INT46_S: u32 = 21;
const NVIC_PRI11_INT45_S: u32 = 13;
const NVIC_PRI11_INT44_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI12 register.
//
// *****************************************************************************
const NVIC_PRI12_INT51_M: u32 = 0xE0000000; // Interrupt 51 Priority Mask
const NVIC_PRI12_INT50_M: u32 = 0x00E00000; // Interrupt 50 Priority Mask
const NVIC_PRI12_INT49_M: u32 = 0x0000E000; // Interrupt 49 Priority Mask
const NVIC_PRI12_INT48_M: u32 = 0x000000E0; // Interrupt 48 Priority Mask
const NVIC_PRI12_INT51_S: u32 = 29;
const NVIC_PRI12_INT50_S: u32 = 21;
const NVIC_PRI12_INT49_S: u32 = 13;
const NVIC_PRI12_INT48_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI13 register.
//
// *****************************************************************************
const NVIC_PRI13_INT55_M: u32 = 0xE0000000; // Interrupt 55 Priority Mask
const NVIC_PRI13_INT54_M: u32 = 0x00E00000; // Interrupt 54 Priority Mask
const NVIC_PRI13_INT53_M: u32 = 0x0000E000; // Interrupt 53 Priority Mask
const NVIC_PRI13_INT52_M: u32 = 0x000000E0; // Interrupt 52 Priority Mask
const NVIC_PRI13_INT55_S: u32 = 29;
const NVIC_PRI13_INT54_S: u32 = 21;
const NVIC_PRI13_INT53_S: u32 = 13;
const NVIC_PRI13_INT52_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI14 register.
//
// *****************************************************************************
const NVIC_PRI14_INTD_M: u32 = 0xE0000000; // Interrupt 59 Priority Mask
const NVIC_PRI14_INTC_M: u32 = 0x00E00000; // Interrupt 58 Priority Mask
const NVIC_PRI14_INTB_M: u32 = 0x0000E000; // Interrupt 57 Priority Mask
const NVIC_PRI14_INTA_M: u32 = 0x000000E0; // Interrupt 56 Priority Mask
const NVIC_PRI14_INTD_S: u32 = 29;
const NVIC_PRI14_INTC_S: u32 = 21;
const NVIC_PRI14_INTB_S: u32 = 13;
const NVIC_PRI14_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI15 register.
//
// *****************************************************************************
const NVIC_PRI15_INTD_M: u32 = 0xE0000000; // Interrupt 63 Priority Mask
const NVIC_PRI15_INTC_M: u32 = 0x00E00000; // Interrupt 62 Priority Mask
const NVIC_PRI15_INTB_M: u32 = 0x0000E000; // Interrupt 61 Priority Mask
const NVIC_PRI15_INTA_M: u32 = 0x000000E0; // Interrupt 60 Priority Mask
const NVIC_PRI15_INTD_S: u32 = 29;
const NVIC_PRI15_INTC_S: u32 = 21;
const NVIC_PRI15_INTB_S: u32 = 13;
const NVIC_PRI15_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI16 register.
//
// *****************************************************************************
const NVIC_PRI16_INTD_M: u32 = 0xE0000000; // Interrupt 67 Priority Mask
const NVIC_PRI16_INTC_M: u32 = 0x00E00000; // Interrupt 66 Priority Mask
const NVIC_PRI16_INTB_M: u32 = 0x0000E000; // Interrupt 65 Priority Mask
const NVIC_PRI16_INTA_M: u32 = 0x000000E0; // Interrupt 64 Priority Mask
const NVIC_PRI16_INTD_S: u32 = 29;
const NVIC_PRI16_INTC_S: u32 = 21;
const NVIC_PRI16_INTB_S: u32 = 13;
const NVIC_PRI16_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI17 register.
//
// *****************************************************************************
const NVIC_PRI17_INTD_M: u32 = 0xE0000000; // Interrupt 71 Priority Mask
const NVIC_PRI17_INTC_M: u32 = 0x00E00000; // Interrupt 70 Priority Mask
const NVIC_PRI17_INTB_M: u32 = 0x0000E000; // Interrupt 69 Priority Mask
const NVIC_PRI17_INTA_M: u32 = 0x000000E0; // Interrupt 68 Priority Mask
const NVIC_PRI17_INTD_S: u32 = 29;
const NVIC_PRI17_INTC_S: u32 = 21;
const NVIC_PRI17_INTB_S: u32 = 13;
const NVIC_PRI17_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI18 register.
//
// *****************************************************************************
const NVIC_PRI18_INTD_M: u32 = 0xE0000000; // Interrupt 75 Priority Mask
const NVIC_PRI18_INTC_M: u32 = 0x00E00000; // Interrupt 74 Priority Mask
const NVIC_PRI18_INTB_M: u32 = 0x0000E000; // Interrupt 73 Priority Mask
const NVIC_PRI18_INTA_M: u32 = 0x000000E0; // Interrupt 72 Priority Mask
const NVIC_PRI18_INTD_S: u32 = 29;
const NVIC_PRI18_INTC_S: u32 = 21;
const NVIC_PRI18_INTB_S: u32 = 13;
const NVIC_PRI18_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI19 register.
//
// *****************************************************************************
const NVIC_PRI19_INTD_M: u32 = 0xE0000000; // Interrupt 79 Priority Mask
const NVIC_PRI19_INTC_M: u32 = 0x00E00000; // Interrupt 78 Priority Mask
const NVIC_PRI19_INTB_M: u32 = 0x0000E000; // Interrupt 77 Priority Mask
const NVIC_PRI19_INTA_M: u32 = 0x000000E0; // Interrupt 76 Priority Mask
const NVIC_PRI19_INTD_S: u32 = 29;
const NVIC_PRI19_INTC_S: u32 = 21;
const NVIC_PRI19_INTB_S: u32 = 13;
const NVIC_PRI19_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI20 register.
//
// *****************************************************************************
const NVIC_PRI20_INTD_M: u32 = 0xE0000000; // Interrupt 83 Priority Mask
const NVIC_PRI20_INTC_M: u32 = 0x00E00000; // Interrupt 82 Priority Mask
const NVIC_PRI20_INTB_M: u32 = 0x0000E000; // Interrupt 81 Priority Mask
const NVIC_PRI20_INTA_M: u32 = 0x000000E0; // Interrupt 80 Priority Mask
const NVIC_PRI20_INTD_S: u32 = 29;
const NVIC_PRI20_INTC_S: u32 = 21;
const NVIC_PRI20_INTB_S: u32 = 13;
const NVIC_PRI20_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI21 register.
//
// *****************************************************************************
const NVIC_PRI21_INTD_M: u32 = 0xE0000000; // Interrupt 87 Priority Mask
const NVIC_PRI21_INTC_M: u32 = 0x00E00000; // Interrupt 86 Priority Mask
const NVIC_PRI21_INTB_M: u32 = 0x0000E000; // Interrupt 85 Priority Mask
const NVIC_PRI21_INTA_M: u32 = 0x000000E0; // Interrupt 84 Priority Mask
const NVIC_PRI21_INTD_S: u32 = 29;
const NVIC_PRI21_INTC_S: u32 = 21;
const NVIC_PRI21_INTB_S: u32 = 13;
const NVIC_PRI21_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI22 register.
//
// *****************************************************************************
const NVIC_PRI22_INTD_M: u32 = 0xE0000000; // Interrupt 91 Priority Mask
const NVIC_PRI22_INTC_M: u32 = 0x00E00000; // Interrupt 90 Priority Mask
const NVIC_PRI22_INTB_M: u32 = 0x0000E000; // Interrupt 89 Priority Mask
const NVIC_PRI22_INTA_M: u32 = 0x000000E0; // Interrupt 88 Priority Mask
const NVIC_PRI22_INTD_S: u32 = 29;
const NVIC_PRI22_INTC_S: u32 = 21;
const NVIC_PRI22_INTB_S: u32 = 13;
const NVIC_PRI22_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI23 register.
//
// *****************************************************************************
const NVIC_PRI23_INTD_M: u32 = 0xE0000000; // Interrupt 95 Priority Mask
const NVIC_PRI23_INTC_M: u32 = 0x00E00000; // Interrupt 94 Priority Mask
const NVIC_PRI23_INTB_M: u32 = 0x0000E000; // Interrupt 93 Priority Mask
const NVIC_PRI23_INTA_M: u32 = 0x000000E0; // Interrupt 92 Priority Mask
const NVIC_PRI23_INTD_S: u32 = 29;
const NVIC_PRI23_INTC_S: u32 = 21;
const NVIC_PRI23_INTB_S: u32 = 13;
const NVIC_PRI23_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI24 register.
//
// *****************************************************************************
const NVIC_PRI24_INTD_M: u32 = 0xE0000000; // Interrupt 99 Priority Mask
const NVIC_PRI24_INTC_M: u32 = 0x00E00000; // Interrupt 98 Priority Mask
const NVIC_PRI24_INTB_M: u32 = 0x0000E000; // Interrupt 97 Priority Mask
const NVIC_PRI24_INTA_M: u32 = 0x000000E0; // Interrupt 96 Priority Mask
const NVIC_PRI24_INTD_S: u32 = 29;
const NVIC_PRI24_INTC_S: u32 = 21;
const NVIC_PRI24_INTB_S: u32 = 13;
const NVIC_PRI24_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI25 register.
//
// *****************************************************************************
const NVIC_PRI25_INTD_M: u32 = 0xE0000000; // Interrupt 103 Priority Mask
const NVIC_PRI25_INTC_M: u32 = 0x00E00000; // Interrupt 102 Priority Mask
const NVIC_PRI25_INTB_M: u32 = 0x0000E000; // Interrupt 101 Priority Mask
const NVIC_PRI25_INTA_M: u32 = 0x000000E0; // Interrupt 100 Priority Mask
const NVIC_PRI25_INTD_S: u32 = 29;
const NVIC_PRI25_INTC_S: u32 = 21;
const NVIC_PRI25_INTB_S: u32 = 13;
const NVIC_PRI25_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI26 register.
//
// *****************************************************************************
const NVIC_PRI26_INTD_M: u32 = 0xE0000000; // Interrupt 107 Priority Mask
const NVIC_PRI26_INTC_M: u32 = 0x00E00000; // Interrupt 106 Priority Mask
const NVIC_PRI26_INTB_M: u32 = 0x0000E000; // Interrupt 105 Priority Mask
const NVIC_PRI26_INTA_M: u32 = 0x000000E0; // Interrupt 104 Priority Mask
const NVIC_PRI26_INTD_S: u32 = 29;
const NVIC_PRI26_INTC_S: u32 = 21;
const NVIC_PRI26_INTB_S: u32 = 13;
const NVIC_PRI26_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI27 register.
//
// *****************************************************************************
const NVIC_PRI27_INTD_M: u32 = 0xE0000000; // Interrupt 111 Priority Mask
const NVIC_PRI27_INTC_M: u32 = 0x00E00000; // Interrupt 110 Priority Mask
const NVIC_PRI27_INTB_M: u32 = 0x0000E000; // Interrupt 109 Priority Mask
const NVIC_PRI27_INTA_M: u32 = 0x000000E0; // Interrupt 108 Priority Mask
const NVIC_PRI27_INTD_S: u32 = 29;
const NVIC_PRI27_INTC_S: u32 = 21;
const NVIC_PRI27_INTB_S: u32 = 13;
const NVIC_PRI27_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI28 register.
//
// *****************************************************************************
const NVIC_PRI28_INTD_M: u32 = 0xE0000000; // Interrupt 115 Priority Mask
const NVIC_PRI28_INTC_M: u32 = 0x00E00000; // Interrupt 114 Priority Mask
const NVIC_PRI28_INTB_M: u32 = 0x0000E000; // Interrupt 113 Priority Mask
const NVIC_PRI28_INTA_M: u32 = 0x000000E0; // Interrupt 112 Priority Mask
const NVIC_PRI28_INTD_S: u32 = 29;
const NVIC_PRI28_INTC_S: u32 = 21;
const NVIC_PRI28_INTB_S: u32 = 13;
const NVIC_PRI28_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI29 register.
//
// *****************************************************************************
const NVIC_PRI29_INTD_M: u32 = 0xE0000000; // Interrupt 119 Priority Mask
const NVIC_PRI29_INTC_M: u32 = 0x00E00000; // Interrupt 118 Priority Mask
const NVIC_PRI29_INTB_M: u32 = 0x0000E000; // Interrupt 117 Priority Mask
const NVIC_PRI29_INTA_M: u32 = 0x000000E0; // Interrupt 116 Priority Mask
const NVIC_PRI29_INTD_S: u32 = 29;
const NVIC_PRI29_INTC_S: u32 = 21;
const NVIC_PRI29_INTB_S: u32 = 13;
const NVIC_PRI29_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI30 register.
//
// *****************************************************************************
const NVIC_PRI30_INTD_M: u32 = 0xE0000000; // Interrupt 123 Priority Mask
const NVIC_PRI30_INTC_M: u32 = 0x00E00000; // Interrupt 122 Priority Mask
const NVIC_PRI30_INTB_M: u32 = 0x0000E000; // Interrupt 121 Priority Mask
const NVIC_PRI30_INTA_M: u32 = 0x000000E0; // Interrupt 120 Priority Mask
const NVIC_PRI30_INTD_S: u32 = 29;
const NVIC_PRI30_INTC_S: u32 = 21;
const NVIC_PRI30_INTB_S: u32 = 13;
const NVIC_PRI30_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI31 register.
//
// *****************************************************************************
const NVIC_PRI31_INTD_M: u32 = 0xE0000000; // Interrupt 127 Priority Mask
const NVIC_PRI31_INTC_M: u32 = 0x00E00000; // Interrupt 126 Priority Mask
const NVIC_PRI31_INTB_M: u32 = 0x0000E000; // Interrupt 125 Priority Mask
const NVIC_PRI31_INTA_M: u32 = 0x000000E0; // Interrupt 124 Priority Mask
const NVIC_PRI31_INTD_S: u32 = 29;
const NVIC_PRI31_INTC_S: u32 = 21;
const NVIC_PRI31_INTB_S: u32 = 13;
const NVIC_PRI31_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI32 register.
//
// *****************************************************************************
const NVIC_PRI32_INTD_M: u32 = 0xE0000000; // Interrupt 131 Priority Mask
const NVIC_PRI32_INTC_M: u32 = 0x00E00000; // Interrupt 130 Priority Mask
const NVIC_PRI32_INTB_M: u32 = 0x0000E000; // Interrupt 129 Priority Mask
const NVIC_PRI32_INTA_M: u32 = 0x000000E0; // Interrupt 128 Priority Mask
const NVIC_PRI32_INTD_S: u32 = 29;
const NVIC_PRI32_INTC_S: u32 = 21;
const NVIC_PRI32_INTB_S: u32 = 13;
const NVIC_PRI32_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI33 register.
//
// *****************************************************************************
const NVIC_PRI33_INTD_M: u32 = 0xE0000000; // Interrupt Priority for Interrupt [4n+3]
const NVIC_PRI33_INTC_M: u32 = 0x00E00000; // Interrupt Priority for Interrupt [4n+2]
const NVIC_PRI33_INTB_M: u32 = 0x0000E000; // Interrupt Priority for Interrupt [4n+1]
const NVIC_PRI33_INTA_M: u32 = 0x000000E0; // Interrupt Priority for Interrupt [4n]
const NVIC_PRI33_INTD_S: u32 = 29;
const NVIC_PRI33_INTC_S: u32 = 21;
const NVIC_PRI33_INTB_S: u32 = 13;
const NVIC_PRI33_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_PRI34 register.
//
// *****************************************************************************
const NVIC_PRI34_INTD_M: u32 = 0xE0000000; // Interrupt Priority for Interrupt [4n+3]
const NVIC_PRI34_INTC_M: u32 = 0x00E00000; // Interrupt Priority for Interrupt [4n+2]
const NVIC_PRI34_INTB_M: u32 = 0x0000E000; // Interrupt Priority for Interrupt [4n+1]
const NVIC_PRI34_INTA_M: u32 = 0x000000E0; // Interrupt Priority for Interrupt [4n]
const NVIC_PRI34_INTD_S: u32 = 29;
const NVIC_PRI34_INTC_S: u32 = 21;
const NVIC_PRI34_INTB_S: u32 = 13;
const NVIC_PRI34_INTA_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_CPUID register.
//
// *****************************************************************************
const NVIC_CPUID_IMP_M: u32 = 0xFF000000; // Implementer Code
const NVIC_CPUID_IMP_ARM: u32 = 0x41000000; // ARM
const NVIC_CPUID_VAR_M: u32 = 0x00F00000; // Variant Number
const NVIC_CPUID_CON_M: u32 = 0x000F0000; // Constant
const NVIC_CPUID_PARTNO_M: u32 = 0x0000FFF0; // Part Number
const NVIC_CPUID_PARTNO_CM4: u32 = 0x0000C240; // Cortex-M4 processor
const NVIC_CPUID_REV_M: u32 = 0x0000000F; // Revision Number

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_INT_CTRL register.
//
// *****************************************************************************
const NVIC_INT_CTRL_NMI_SET: u32 = 0x80000000; // NMI Set Pending
const NVIC_INT_CTRL_PEND_SV: u32 = 0x10000000; // PendSV Set Pending
const NVIC_INT_CTRL_UNPEND_SV: u32 = 0x08000000; // PendSV Clear Pending
const NVIC_INT_CTRL_PENDSTSET: u32 = 0x04000000; // SysTick Set Pending
const NVIC_INT_CTRL_PENDSTCLR: u32 = 0x02000000; // SysTick Clear Pending
const NVIC_INT_CTRL_ISR_PRE: u32 = 0x00800000; // Debug Interrupt Handling
const NVIC_INT_CTRL_ISR_PEND: u32 = 0x00400000; // Interrupt Pending
const NVIC_INT_CTRL_VEC_PEN_M: u32 = 0x000FF000; // Interrupt Pending Vector Number
const NVIC_INT_CTRL_VEC_PEN_NMI: u32 = 0x00002000; // NMI
const NVIC_INT_CTRL_VEC_PEN_HARD: u32 = 0x00003000; // Hard fault
const NVIC_INT_CTRL_VEC_PEN_MEM: u32 = 0x00004000; // Memory management fault
const NVIC_INT_CTRL_VEC_PEN_BUS: u32 = 0x00005000; // Bus fault
const NVIC_INT_CTRL_VEC_PEN_USG: u32 = 0x00006000; // Usage fault
const NVIC_INT_CTRL_VEC_PEN_SVC: u32 = 0x0000B000; // SVCall
const NVIC_INT_CTRL_VEC_PEN_PNDSV: u32 = 0x0000E000; // PendSV
const NVIC_INT_CTRL_VEC_PEN_TICK: u32 = 0x0000F000; // SysTick
const NVIC_INT_CTRL_RET_BASE: u32 = 0x00000800; // Return to Base
const NVIC_INT_CTRL_VEC_ACT_M: u32 = 0x000000FF; // Interrupt Pending Vector Number
const NVIC_INT_CTRL_VEC_ACT_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_VTABLE register.
//
// *****************************************************************************
const NVIC_VTABLE_BASE: u32 = 0x20000000; // Vector Table Base
const NVIC_VTABLE_OFFSET_M: u32 = 0x1FFFFC00; // Vector Table Offset
const NVIC_VTABLE_OFFSET_S: u32 = 10;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_APINT register.
//
// *****************************************************************************
const NVIC_APINT_VECTKEY_M: u32 = 0xFFFF0000; // Register Key
const NVIC_APINT_VECTKEY: u32 = 0x05FA0000; // Vector key
const NVIC_APINT_ENDIANESS: u32 = 0x00008000; // Data Endianess
const NVIC_APINT_PRIGROUP_M: u32 = 0x00000700; // Interrupt Priority Grouping
const NVIC_APINT_PRIGROUP_7_1: u32 = 0x00000000; // Priority group 7.1 split
const NVIC_APINT_PRIGROUP_6_2: u32 = 0x00000100; // Priority group 6.2 split
const NVIC_APINT_PRIGROUP_5_3: u32 = 0x00000200; // Priority group 5.3 split
const NVIC_APINT_PRIGROUP_4_4: u32 = 0x00000300; // Priority group 4.4 split
const NVIC_APINT_PRIGROUP_3_5: u32 = 0x00000400; // Priority group 3.5 split
const NVIC_APINT_PRIGROUP_2_6: u32 = 0x00000500; // Priority group 2.6 split
const NVIC_APINT_PRIGROUP_1_7: u32 = 0x00000600; // Priority group 1.7 split
const NVIC_APINT_PRIGROUP_0_8: u32 = 0x00000700; // Priority group 0.8 split
const NVIC_APINT_SYSRESETREQ: u32 = 0x00000004; // System Reset Request
const NVIC_APINT_VECT_CLR_ACT: u32 = 0x00000002; // Clear Active NMI / Fault
const NVIC_APINT_VECT_RESET: u32 = 0x00000001; // System Reset

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SYS_CTRL register.
//
// *****************************************************************************
const NVIC_SYS_CTRL_SEVONPEND: u32 = 0x00000010; // Wake Up on Pending
const NVIC_SYS_CTRL_SLEEPDEEP: u32 = 0x00000004; // Deep Sleep Enable
const NVIC_SYS_CTRL_SLEEPEXIT: u32 = 0x00000002; // Sleep on ISR Exit

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_CFG_CTRL register.
//
// *****************************************************************************
const NVIC_CFG_CTRL_STKALIGN: u32 = 0x00000200; // Stack Alignment on Exception Entry
const NVIC_CFG_CTRL_BFHFNMIGN: u32 = 0x00000100; // Ignore Bus Fault in NMI and Fault
const NVIC_CFG_CTRL_DIV0: u32 = 0x00000010; // Trap on Divide by 0
const NVIC_CFG_CTRL_UNALIGNED: u32 = 0x00000008; // Trap on Unaligned Access
const NVIC_CFG_CTRL_MAIN_PEND: u32 = 0x00000002; // Allow Main Interrupt Trigger
const NVIC_CFG_CTRL_BASE_THR: u32 = 0x00000001; // Thread State Control

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SYS_PRI1 register.
//
// *****************************************************************************
const NVIC_SYS_PRI1_USAGE_M: u32 = 0x00E00000; // Usage Fault Priority
const NVIC_SYS_PRI1_BUS_M: u32 = 0x0000E000; // Bus Fault Priority
const NVIC_SYS_PRI1_MEM_M: u32 = 0x000000E0; // Memory Management Fault Priority
const NVIC_SYS_PRI1_USAGE_S: u32 = 21;
const NVIC_SYS_PRI1_BUS_S: u32 = 13;
const NVIC_SYS_PRI1_MEM_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SYS_PRI2 register.
//
// *****************************************************************************
const NVIC_SYS_PRI2_SVC_M: u32 = 0xE0000000; // SVCall Priority
const NVIC_SYS_PRI2_SVC_S: u32 = 29;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SYS_PRI3 register.
//
// *****************************************************************************
const NVIC_SYS_PRI3_TICK_M: u32 = 0xE0000000; // SysTick Exception Priority
const NVIC_SYS_PRI3_PENDSV_M: u32 = 0x00E00000; // PendSV Priority
const NVIC_SYS_PRI3_DEBUG_M: u32 = 0x000000E0; // Debug Priority
const NVIC_SYS_PRI3_TICK_S: u32 = 29;
const NVIC_SYS_PRI3_PENDSV_S: u32 = 21;
const NVIC_SYS_PRI3_DEBUG_S: u32 = 5;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SYS_HND_CTRL
// register.
//
// *****************************************************************************
const NVIC_SYS_HND_CTRL_USAGE: u32 = 0x00040000; // Usage Fault Enable
const NVIC_SYS_HND_CTRL_BUS: u32 = 0x00020000; // Bus Fault Enable
const NVIC_SYS_HND_CTRL_MEM: u32 = 0x00010000; // Memory Management Fault Enable
const NVIC_SYS_HND_CTRL_SVC: u32 = 0x00008000; // SVC Call Pending
const NVIC_SYS_HND_CTRL_BUSP: u32 = 0x00004000; // Bus Fault Pending
const NVIC_SYS_HND_CTRL_MEMP: u32 = 0x00002000; // Memory Management Fault Pending
const NVIC_SYS_HND_CTRL_USAGEP: u32 = 0x00001000; // Usage Fault Pending
const NVIC_SYS_HND_CTRL_TICK: u32 = 0x00000800; // SysTick Exception Active
const NVIC_SYS_HND_CTRL_PNDSV: u32 = 0x00000400; // PendSV Exception Active
const NVIC_SYS_HND_CTRL_MON: u32 = 0x00000100; // Debug Monitor Active
const NVIC_SYS_HND_CTRL_SVCA: u32 = 0x00000080; // SVC Call Active
const NVIC_SYS_HND_CTRL_USGA: u32 = 0x00000008; // Usage Fault Active
const NVIC_SYS_HND_CTRL_BUSA: u32 = 0x00000002; // Bus Fault Active
const NVIC_SYS_HND_CTRL_MEMA: u32 = 0x00000001; // Memory Management Fault Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_FAULT_STAT
// register.
//
// *****************************************************************************
const NVIC_FAULT_STAT_DIV0: u32 = 0x02000000; // Divide-by-Zero Usage Fault
const NVIC_FAULT_STAT_UNALIGN: u32 = 0x01000000; // Unaligned Access Usage Fault
const NVIC_FAULT_STAT_NOCP: u32 = 0x00080000; // No Coprocessor Usage Fault
const NVIC_FAULT_STAT_INVPC: u32 = 0x00040000; // Invalid PC Load Usage Fault
const NVIC_FAULT_STAT_INVSTAT: u32 = 0x00020000; // Invalid State Usage Fault
const NVIC_FAULT_STAT_UNDEF: u32 = 0x00010000; // Undefined Instruction Usage Fault
const NVIC_FAULT_STAT_BFARV: u32 = 0x00008000; // Bus Fault Address Register Valid
const NVIC_FAULT_STAT_BLSPERR: u32 = 0x00002000; // Bus Fault on Floating-Point Lazy State Preservation
const NVIC_FAULT_STAT_BSTKE: u32 = 0x00001000; // Stack Bus Fault
const NVIC_FAULT_STAT_BUSTKE: u32 = 0x00000800; // Unstack Bus Fault
const NVIC_FAULT_STAT_IMPRE: u32 = 0x00000400; // Imprecise Data Bus Error
const NVIC_FAULT_STAT_PRECISE: u32 = 0x00000200; // Precise Data Bus Error
const NVIC_FAULT_STAT_IBUS: u32 = 0x00000100; // Instruction Bus Error
const NVIC_FAULT_STAT_MMARV: u32 = 0x00000080; // Memory Management Fault Address Register Valid
const NVIC_FAULT_STAT_MLSPERR: u32 = 0x00000020; // Memory Management Fault on Floating-Point Lazy State Preservation
const NVIC_FAULT_STAT_MSTKE: u32 = 0x00000010; // Stack Access Violation
const NVIC_FAULT_STAT_MUSTKE: u32 = 0x00000008; // Unstack Access Violation
const NVIC_FAULT_STAT_DERR: u32 = 0x00000002; // Data Access Violation
const NVIC_FAULT_STAT_IERR: u32 = 0x00000001; // Instruction Access Violation

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_HFAULT_STAT
// register.
//
// *****************************************************************************
const NVIC_HFAULT_STAT_DBG: u32 = 0x80000000; // Debug Event
const NVIC_HFAULT_STAT_FORCED: u32 = 0x40000000; // Forced Hard Fault
const NVIC_HFAULT_STAT_VECT: u32 = 0x00000002; // Vector Table Read Fault

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DEBUG_STAT
// register.
//
// *****************************************************************************
const NVIC_DEBUG_STAT_EXTRNL: u32 = 0x00000010; // EDBGRQ asserted
const NVIC_DEBUG_STAT_VCATCH: u32 = 0x00000008; // Vector catch
const NVIC_DEBUG_STAT_DWTTRAP: u32 = 0x00000004; // DWT match
const NVIC_DEBUG_STAT_BKPT: u32 = 0x00000002; // Breakpoint instruction
const NVIC_DEBUG_STAT_HALTED: u32 = 0x00000001; // Halt request

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MM_ADDR register.
//
// *****************************************************************************
const NVIC_MM_ADDR_M: u32 = 0xFFFFFFFF; // Fault Address
const NVIC_MM_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_FAULT_ADDR
// register.
//
// *****************************************************************************
const NVIC_FAULT_ADDR_M: u32 = 0xFFFFFFFF; // Fault Address
const NVIC_FAULT_ADDR_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_CPAC register.
//
// *****************************************************************************
const NVIC_CPAC_CP11_M: u32 = 0x00C00000; // CP11 Coprocessor Access Privilege
const NVIC_CPAC_CP11_DIS: u32 = 0x00000000; // Access Denied
const NVIC_CPAC_CP11_PRIV: u32 = 0x00400000; // Privileged Access Only
const NVIC_CPAC_CP11_FULL: u32 = 0x00C00000; // Full Access
const NVIC_CPAC_CP10_M: u32 = 0x00300000; // CP10 Coprocessor Access Privilege
const NVIC_CPAC_CP10_DIS: u32 = 0x00000000; // Access Denied
const NVIC_CPAC_CP10_PRIV: u32 = 0x00100000; // Privileged Access Only
const NVIC_CPAC_CP10_FULL: u32 = 0x00300000; // Full Access

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_TYPE register.
//
// *****************************************************************************
const NVIC_MPU_TYPE_IREGION_M: u32 = 0x00FF0000; // Number of I Regions
const NVIC_MPU_TYPE_DREGION_M: u32 = 0x0000FF00; // Number of D Regions
const NVIC_MPU_TYPE_SEPARATE: u32 = 0x00000001; // Separate or Unified MPU
const NVIC_MPU_TYPE_IREGION_S: u32 = 16;
const NVIC_MPU_TYPE_DREGION_S: u32 = 8;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_CTRL register.
//
// *****************************************************************************
const NVIC_MPU_CTRL_PRIVDEFEN: u32 = 0x00000004; // MPU Default Region
const NVIC_MPU_CTRL_HFNMIENA: u32 = 0x00000002; // MPU Enabled During Faults
const NVIC_MPU_CTRL_ENABLE: u32 = 0x00000001; // MPU Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_NUMBER
// register.
//
// *****************************************************************************
const NVIC_MPU_NUMBER_M: u32 = 0x00000007; // MPU Region to Access
const NVIC_MPU_NUMBER_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_BASE register.
//
// *****************************************************************************
const NVIC_MPU_BASE_ADDR_M: u32 = 0xFFFFFFE0; // Base Address Mask
const NVIC_MPU_BASE_VALID: u32 = 0x00000010; // Region Number Valid
const NVIC_MPU_BASE_REGION_M: u32 = 0x00000007; // Region Number
const NVIC_MPU_BASE_ADDR_S: u32 = 5;
const NVIC_MPU_BASE_REGION_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_ATTR register.
//
// *****************************************************************************
const NVIC_MPU_ATTR_XN: u32 = 0x10000000; // Instruction Access Disable
const NVIC_MPU_ATTR_AP_M: u32 = 0x07000000; // Access Privilege
const NVIC_MPU_ATTR_TEX_M: u32 = 0x00380000; // Type Extension Mask
const NVIC_MPU_ATTR_SHAREABLE: u32 = 0x00040000; // Shareable
const NVIC_MPU_ATTR_CACHEABLE: u32 = 0x00020000; // Cacheable
const NVIC_MPU_ATTR_BUFFRABLE: u32 = 0x00010000; // Bufferable
const NVIC_MPU_ATTR_SRD_M: u32 = 0x0000FF00; // Subregion Disable Bits
const NVIC_MPU_ATTR_SIZE_M: u32 = 0x0000003E; // Region Size Mask
const NVIC_MPU_ATTR_ENABLE: u32 = 0x00000001; // Region Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_BASE1 register.
//
// *****************************************************************************
const NVIC_MPU_BASE1_ADDR_M: u32 = 0xFFFFFFE0; // Base Address Mask
const NVIC_MPU_BASE1_VALID: u32 = 0x00000010; // Region Number Valid
const NVIC_MPU_BASE1_REGION_M: u32 = 0x00000007; // Region Number
const NVIC_MPU_BASE1_ADDR_S: u32 = 5;
const NVIC_MPU_BASE1_REGION_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_ATTR1 register.
//
// *****************************************************************************
const NVIC_MPU_ATTR1_XN: u32 = 0x10000000; // Instruction Access Disable
const NVIC_MPU_ATTR1_AP_M: u32 = 0x07000000; // Access Privilege
const NVIC_MPU_ATTR1_TEX_M: u32 = 0x00380000; // Type Extension Mask
const NVIC_MPU_ATTR1_SHAREABLE: u32 = 0x00040000; // Shareable
const NVIC_MPU_ATTR1_CACHEABLE: u32 = 0x00020000; // Cacheable
const NVIC_MPU_ATTR1_BUFFRABLE: u32 = 0x00010000; // Bufferable
const NVIC_MPU_ATTR1_SRD_M: u32 = 0x0000FF00; // Subregion Disable Bits
const NVIC_MPU_ATTR1_SIZE_M: u32 = 0x0000003E; // Region Size Mask
const NVIC_MPU_ATTR1_ENABLE: u32 = 0x00000001; // Region Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_BASE2 register.
//
// *****************************************************************************
const NVIC_MPU_BASE2_ADDR_M: u32 = 0xFFFFFFE0; // Base Address Mask
const NVIC_MPU_BASE2_VALID: u32 = 0x00000010; // Region Number Valid
const NVIC_MPU_BASE2_REGION_M: u32 = 0x00000007; // Region Number
const NVIC_MPU_BASE2_ADDR_S: u32 = 5;
const NVIC_MPU_BASE2_REGION_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_ATTR2 register.
//
// *****************************************************************************
const NVIC_MPU_ATTR2_XN: u32 = 0x10000000; // Instruction Access Disable
const NVIC_MPU_ATTR2_AP_M: u32 = 0x07000000; // Access Privilege
const NVIC_MPU_ATTR2_TEX_M: u32 = 0x00380000; // Type Extension Mask
const NVIC_MPU_ATTR2_SHAREABLE: u32 = 0x00040000; // Shareable
const NVIC_MPU_ATTR2_CACHEABLE: u32 = 0x00020000; // Cacheable
const NVIC_MPU_ATTR2_BUFFRABLE: u32 = 0x00010000; // Bufferable
const NVIC_MPU_ATTR2_SRD_M: u32 = 0x0000FF00; // Subregion Disable Bits
const NVIC_MPU_ATTR2_SIZE_M: u32 = 0x0000003E; // Region Size Mask
const NVIC_MPU_ATTR2_ENABLE: u32 = 0x00000001; // Region Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_BASE3 register.
//
// *****************************************************************************
const NVIC_MPU_BASE3_ADDR_M: u32 = 0xFFFFFFE0; // Base Address Mask
const NVIC_MPU_BASE3_VALID: u32 = 0x00000010; // Region Number Valid
const NVIC_MPU_BASE3_REGION_M: u32 = 0x00000007; // Region Number
const NVIC_MPU_BASE3_ADDR_S: u32 = 5;
const NVIC_MPU_BASE3_REGION_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_MPU_ATTR3 register.
//
// *****************************************************************************
const NVIC_MPU_ATTR3_XN: u32 = 0x10000000; // Instruction Access Disable
const NVIC_MPU_ATTR3_AP_M: u32 = 0x07000000; // Access Privilege
const NVIC_MPU_ATTR3_TEX_M: u32 = 0x00380000; // Type Extension Mask
const NVIC_MPU_ATTR3_SHAREABLE: u32 = 0x00040000; // Shareable
const NVIC_MPU_ATTR3_CACHEABLE: u32 = 0x00020000; // Cacheable
const NVIC_MPU_ATTR3_BUFFRABLE: u32 = 0x00010000; // Bufferable
const NVIC_MPU_ATTR3_SRD_M: u32 = 0x0000FF00; // Subregion Disable Bits
const NVIC_MPU_ATTR3_SIZE_M: u32 = 0x0000003E; // Region Size Mask
const NVIC_MPU_ATTR3_ENABLE: u32 = 0x00000001; // Region Enable

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DBG_CTRL register.
//
// *****************************************************************************
const NVIC_DBG_CTRL_DBGKEY_M: u32 = 0xFFFF0000; // Debug key mask
const NVIC_DBG_CTRL_DBGKEY: u32 = 0xA05F0000; // Debug key
const NVIC_DBG_CTRL_S_RESET_ST: u32 = 0x02000000; // Core has reset since last read
const NVIC_DBG_CTRL_S_RETIRE_ST: u32 = 0x01000000; // Core has executed insruction since last read
const NVIC_DBG_CTRL_S_LOCKUP: u32 = 0x00080000; // Core is locked up
const NVIC_DBG_CTRL_S_SLEEP: u32 = 0x00040000; // Core is sleeping
const NVIC_DBG_CTRL_S_HALT: u32 = 0x00020000; // Core status on halt
const NVIC_DBG_CTRL_S_REGRDY: u32 = 0x00010000; // Register read/write available
const NVIC_DBG_CTRL_C_SNAPSTALL: u32 = 0x00000020; // Breaks a stalled load/store
const NVIC_DBG_CTRL_C_MASKINT: u32 = 0x00000008; // Mask interrupts when stepping
const NVIC_DBG_CTRL_C_STEP: u32 = 0x00000004; // Step the core
const NVIC_DBG_CTRL_C_HALT: u32 = 0x00000002; // Halt the core
const NVIC_DBG_CTRL_C_DEBUGEN: u32 = 0x00000001; // Enable debug

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DBG_XFER register.
//
// *****************************************************************************
const NVIC_DBG_XFER_REG_WNR: u32 = 0x00010000; // Write or not read
const NVIC_DBG_XFER_REG_SEL_M: u32 = 0x0000001F; // Register
const NVIC_DBG_XFER_REG_R0: u32 = 0x00000000; // Register R0
const NVIC_DBG_XFER_REG_R1: u32 = 0x00000001; // Register R1
const NVIC_DBG_XFER_REG_R2: u32 = 0x00000002; // Register R2
const NVIC_DBG_XFER_REG_R3: u32 = 0x00000003; // Register R3
const NVIC_DBG_XFER_REG_R4: u32 = 0x00000004; // Register R4
const NVIC_DBG_XFER_REG_R5: u32 = 0x00000005; // Register R5
const NVIC_DBG_XFER_REG_R6: u32 = 0x00000006; // Register R6
const NVIC_DBG_XFER_REG_R7: u32 = 0x00000007; // Register R7
const NVIC_DBG_XFER_REG_R8: u32 = 0x00000008; // Register R8
const NVIC_DBG_XFER_REG_R9: u32 = 0x00000009; // Register R9
const NVIC_DBG_XFER_REG_R10: u32 = 0x0000000A; // Register R10
const NVIC_DBG_XFER_REG_R11: u32 = 0x0000000B; // Register R11
const NVIC_DBG_XFER_REG_R12: u32 = 0x0000000C; // Register R12
const NVIC_DBG_XFER_REG_R13: u32 = 0x0000000D; // Register R13
const NVIC_DBG_XFER_REG_R14: u32 = 0x0000000E; // Register R14
const NVIC_DBG_XFER_REG_R15: u32 = 0x0000000F; // Register R15
const NVIC_DBG_XFER_REG_FLAGS: u32 = 0x00000010; // xPSR/Flags register
const NVIC_DBG_XFER_REG_MSP: u32 = 0x00000011; // Main SP
const NVIC_DBG_XFER_REG_PSP: u32 = 0x00000012; // Process SP
const NVIC_DBG_XFER_REG_DSP: u32 = 0x00000013; // Deep SP
const NVIC_DBG_XFER_REG_CFBP: u32 = 0x00000014; // Control/Fault/BasePri/PriMask

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DBG_DATA register.
//
// *****************************************************************************
const NVIC_DBG_DATA_M: u32 = 0xFFFFFFFF; // Data temporary cache
const NVIC_DBG_DATA_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_DBG_INT register.
//
// *****************************************************************************
const NVIC_DBG_INT_HARDERR: u32 = 0x00000400; // Debug trap on hard fault
const NVIC_DBG_INT_INTERR: u32 = 0x00000200; // Debug trap on interrupt errors
const NVIC_DBG_INT_BUSERR: u32 = 0x00000100; // Debug trap on bus error
const NVIC_DBG_INT_STATERR: u32 = 0x00000080; // Debug trap on usage fault state
const NVIC_DBG_INT_CHKERR: u32 = 0x00000040; // Debug trap on usage fault check
const NVIC_DBG_INT_NOCPERR: u32 = 0x00000020; // Debug trap on coprocessor error
const NVIC_DBG_INT_MMERR: u32 = 0x00000010; // Debug trap on mem manage fault
const NVIC_DBG_INT_RESET: u32 = 0x00000008; // Core reset status
const NVIC_DBG_INT_RSTPENDCLR: u32 = 0x00000004; // Clear pending core reset
const NVIC_DBG_INT_RSTPENDING: u32 = 0x00000002; // Core reset is pending
const NVIC_DBG_INT_RSTVCATCH: u32 = 0x00000001; // Reset vector catch

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_SW_TRIG register.
//
// *****************************************************************************
const NVIC_SW_TRIG_INTID_M: u32 = 0x000000FF; // Interrupt ID
const NVIC_SW_TRIG_INTID_S: u32 = 0;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_FPCC register.
//
// *****************************************************************************
const NVIC_FPCC_ASPEN: u32 = 0x80000000; // Automatic State Preservation Enable
const NVIC_FPCC_LSPEN: u32 = 0x40000000; // Lazy State Preservation Enable
const NVIC_FPCC_MONRDY: u32 = 0x00000100; // Monitor Ready
const NVIC_FPCC_BFRDY: u32 = 0x00000040; // Bus Fault Ready
const NVIC_FPCC_MMRDY: u32 = 0x00000020; // Memory Management Fault Ready
const NVIC_FPCC_HFRDY: u32 = 0x00000010; // Hard Fault Ready
const NVIC_FPCC_THREAD: u32 = 0x00000008; // Thread Mode
const NVIC_FPCC_USER: u32 = 0x00000002; // User Privilege Level
const NVIC_FPCC_LSPACT: u32 = 0x00000001; // Lazy State Preservation Active

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_FPCA register.
//
// *****************************************************************************
const NVIC_FPCA_ADDRESS_M: u32 = 0xFFFFFFF8; // Address
const NVIC_FPCA_ADDRESS_S: u32 = 3;

// *****************************************************************************
//
// The following are defines for the bit fields in the NVIC_FPDSC register.
//
// *****************************************************************************
const NVIC_FPDSC_AHP: u32 = 0x04000000; // AHP Bit Default
const NVIC_FPDSC_DN: u32 = 0x02000000; // DN Bit Default
const NVIC_FPDSC_FZ: u32 = 0x01000000; // FZ Bit Default
const NVIC_FPDSC_RMODE_M: u32 = 0x00C00000; // RMODE Bit Default
const NVIC_FPDSC_RMODE_RN: u32 = 0x00000000; // Round to Nearest (RN) mode
const NVIC_FPDSC_RMODE_RP: u32 = 0x00400000; // Round towards Plus Infinity (RP) mode
const NVIC_FPDSC_RMODE_RM: u32 = 0x00800000; // Round towards Minus Infinity (RM) mode
const NVIC_FPDSC_RMODE_RZ: u32 = 0x00C00000; // Round towards Zero (RZ) mode
