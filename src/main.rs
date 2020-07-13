//  vim:set ts=4 sw=4 sts=0 fileencoding=utf-8:
//  ----------------------------------------------------------------------------
/*
    @file       $Id$
    @author     zuntan
*/
//  ----------------------------------------------------------------------------


use std::env;
use std::io::{ BufWriter };
use std::fs::{ File, metadata };
use std::str::FromStr;

use num_traits::{ Float, FloatConst, NumAssign, FromPrimitive, ToPrimitive };

use hound;
use chfft::CFft1D;
use num_complex::Complex;

use chrono::{ Utc };

const EX_USAGE      : i32 = 64;
/*
const PKG_NAME      : &'static str = env!("CARGO_PKG_NAME");
*/
const PKG_VERSION   : &'static str = env!("CARGO_PKG_VERSION");

fn usage( prog: &str, opts: getopts::Options )
{
    eprintln!( "{}", opts.usage( &format!("Usage: {} [options]", prog ) ) );
}

struct ProgOpt
{
    wav             : String
,   wav_bw          : String
,   len             : u32
,   f64             : bool
,   skip_fft        : bool
,   enable_fftbw    : bool
}

fn parse_opt() -> ( ProgOpt, Vec::< f32 > )
{
    let args: Vec< String > = env::args().collect();

    let arg_prog = args[0].clone();     // arg_prog

    let mut opts = getopts::Options::new();

    opts.optflagopt( "w", "wav", "wav file output", "tmp.wav" );
    opts.optflagopt( "x", "fftbw_wav", "wav file output (fft backword)", "tmp_fftbw.wav" );
    opts.optopt( "l", "len", "wav file output len (sec)", "15" );
    opts.optmulti( "f", "freq", "frequency (20-20000)", "1000" );
    opts.optflag( "6", "f64", "use f64 insteed of f32" );
    opts.optflag( "s", "skip_fft", "skip fft" );
    opts.optflag( "b", "enable_fft_backward", "enable fft_backward" );
    opts.optflag( "v", "version", "Print version info and exit." );
    opts.optflag( "", "help", "Print this help menu." );

    let opt_matches = match opts.parse( &args[1..] )
    {
        Ok( m ) => { m }
        Err( f ) =>
        {
            eprintln!( "{}", f.to_string() );
            usage( &arg_prog, opts );
            std::process::exit( EX_USAGE );
        }
    };

    if opt_matches.opt_present( "help" )
    {
        usage( &arg_prog, opts );
        std::process::exit( EX_USAGE );
    }

    if opt_matches.opt_present( "version" )
    {
        eprintln!( "{} {}", &arg_prog, PKG_VERSION );
        std::process::exit( EX_USAGE );
    }

    let opt_wav =
        if opt_matches.opt_present( "wav" )
        {
            match opt_matches.opt_str( "wav" )
            {
                Some( x )   => { x }
            ,   None        => { String::from( "tmp.wav" ) }
            }
        }
        else
        {
            String::new()
        };

    let skip_fft        = opt_matches.opt_present( "skip_fft" );
    let enable_fftbw    = opt_matches.opt_present( "enable_fft_backward" ) && !skip_fft;

    let opt_wav_bw =
        if enable_fftbw && opt_matches.opt_present( "fftbw_wav" )
        {
            match opt_matches.opt_str( "fftbw_wav" )
            {
                Some( x )   => { x }
            ,   None        => { String::from( "tmp_fftbw.wav" ) }
            }
        }
        else
        {
            String::new()
        };

    let opt_len : u32 =
        match opt_matches.opt_str( "len" )
        {
            Some( x )   =>
            {
                match u32::from_str( &x )
                {
                    Err( e )    =>
                    {
                        eprintln!( "option --len {} *** {:?}", x, e );
                        usage( &arg_prog, opts );
                        std::process::exit( EX_USAGE );
                    }
                ,   Ok( x )     => { x }
                }
            }
        ,   None        => { 15 }
        };


    let opt_freq = opt_matches.opt_strs( "freq" );

    let mut freqs = Vec::< f32 >::new();

    for x in &opt_freq
    {
        match f32::from_str( x )
        {
            Err( e ) =>
            {
                eprintln!( "option --freq {} *** {:?}", x, e );
                usage( &arg_prog, opts );
                std::process::exit( EX_USAGE );
            }
        ,   Ok( x ) if x < 20.0 || x > 20000.0 =>
            {
                eprintln!( "option --freq {} *** range error", x );
                usage( &arg_prog, opts );
                std::process::exit( EX_USAGE );
            }
        ,   Ok( x ) =>
            {
                freqs.push( x );
            }
        }
    }

    if freqs.is_empty()
    {
        freqs.push( 440.000 );      //  A4  440.000
        freqs.push( 554.365 );      //  C#5 554.365
        freqs.push( 659.255 );      //  E5  659.255
    }

    let f64     = opt_matches.opt_present( "f64" );

    ( ProgOpt{ wav : opt_wav, wav_bw : opt_wav_bw, len : opt_len, f64, skip_fft, enable_fftbw }, freqs )
}

trait FloatConstEx
{
    fn c_0_5() -> Self;
    fn c_0_54() -> Self;
    fn c_0_46() -> Self;
    fn c_1_0() -> Self;
    fn c_2_0() -> Self;
}

impl FloatConstEx for f32
{
    fn c_0_5()  -> Self{ 0.5 }
    fn c_0_54() -> Self{ 0.54 }
    fn c_0_46() -> Self{ 0.46 }
    fn c_1_0()  -> Self{ 1.0 }
    fn c_2_0()  -> Self{ 2.0 }
}

impl FloatConstEx for f64
{
    fn c_0_5()  -> Self{ 0.5 }
    fn c_0_54() -> Self{ 0.54 }
    fn c_0_46() -> Self{ 0.46 }
    fn c_1_0()  -> Self{ 1.0 }
    fn c_2_0()  -> Self{ 2.0 }
}

fn sample< T : Float + FloatConst + FromPrimitive + FloatConstEx >( freq : T, t : T ) -> T
{
    ( t * freq * T::c_2_0() * T::PI() ).sin()
}

fn sample_a< T : Float + FloatConst + FromPrimitive + FloatConstEx >( freq : &[ T ], t : T ) -> T
{
    freq.iter().map( | x | sample::<T>( *x, t ) ).fold( num_traits::zero::<T>(), | acc, x | acc + x )
}

const SAMPLE_RATE   : u32   = 44100;
const FFT_LEN       : usize = 4096;

fn bench<
    T : Float + FloatConst + FromPrimitive + ToPrimitive + FloatConstEx + NumAssign + std::fmt::Debug
>( opt : ProgOpt, freqs : &[ T ] )
    -> bool
{
    let tm_st = Utc::now();

    log::info!( "f64           [{}]",         opt.f64 );
    log::info!( "skip_fft      [{}]",         opt.skip_fft );
    log::info!( "wav output    [{}]",         opt.wav );
    log::info!( "enable_fft_bw [{}]",         opt.enable_fftbw );
    log::info!( "wav output bw [{}]",         opt.wav_bw );
    log::info!( "wav len       [{}] sec",     opt.len );
    log::info!( "freqs         {:?}",         freqs );

    let t_sample_rate   = T::from( SAMPLE_RATE ).unwrap();
    let t_i16_max       = T::from( i16::MAX ).unwrap();

    let s_max = ( 0 .. SAMPLE_RATE )
                .map( |x| T::from( x ).unwrap() / t_sample_rate )
                .map( |x| sample_a( freqs, x ) )
                .fold( T::nan(), | m, v | v.max( m ) )
                ;

    let s_max_a = s_max.max( T::from( 3.0 ).unwrap() );

    log::info!( "wav amp     [{:?}]",   s_max_a );

    let spec = hound::WavSpec
    {
        channels        : 1
    ,   sample_rate     : SAMPLE_RATE
    ,   bits_per_sample : 16
    ,   sample_format   : hound::SampleFormat::Int
    };

    let mut writer      : Option::< hound::WavWriter< BufWriter< File > > > = None;
    let mut writer_bw   : Option::< hound::WavWriter< BufWriter< File > > > = None;

    if opt.wav != ""
    {
        writer = match hound::WavWriter::create( &opt.wav, spec )
        {
            Ok( x )     => { Some( x ) }
        ,   Err( x )    =>
            {
                log::error!( "{:?}", x );
                return false;
            }
        };
    }

    if opt.enable_fftbw && opt.wav_bw != ""
    {
        writer_bw = match hound::WavWriter::create( &opt.wav_bw, spec )
        {
            Ok( x )     => { Some( x ) }
        ,   Err( x )    =>
            {
                log::error!( "{:?}", x );
                return false;
            }
        };
    }

    let mut smp_buf     = [ 0_i16 ; FFT_LEN ];
    let mut win_buf     = [ num_traits::zero::<T>() ; FFT_LEN ];

    let mut fft         = CFft1D::< T >::with_len( FFT_LEN );
    let mut fft_buf     = [ Complex::< T >::new( num_traits::zero::<T>(), num_traits::zero::<T>() ) ; FFT_LEN ];
    let mut fft_buf_r   = [ Complex::< T >::new( num_traits::zero::<T>(), num_traits::zero::<T>() ) ; FFT_LEN ];

    let _c_fft_len = T::from( FFT_LEN ).unwrap();

    for tt in 0..FFT_LEN
    {
        // Hanning Window
        //win_buf[ tt ] = T::c_0_5()  - T::c_0_5()  * ( T::PI() * T::c_2_0() * ( T::from( tt ).unwrap() / _c_fft_len ) ).cos();

        // Hamming Window
        // win_buf[ tt ] = T::c_0_54() - T::c_0_46() * ( T::PI() * T::c_2_0() * ( T::from( tt ).unwrap() / _c_fft_len ) ).cos();

        // No Window
        win_buf[ tt ] = T::c_1_0();
    }

    let t_end   : usize = opt.len as usize * SAMPLE_RATE as usize;
    let mut t   : usize = 0;
    let mut ls  : usize = 0;
    let mut c   : usize = 0;

    loop
    {
        for tt in 0..FFT_LEN
        {
            let s = sample_a( freqs, T::from( t + tt ).unwrap() / t_sample_rate );
            let s = s / s_max_a;

            fft_buf[ tt ].re = s * win_buf[ tt ];
            fft_buf[ tt ].im = num_traits::zero::<T>();

            let s = s * t_i16_max;
            let s = s.to_i16().unwrap();

            smp_buf[ tt ] = s;
        }

        if let Some( ref mut writer ) = writer
        {
            let mut s16_writer = writer.get_i16_writer( FFT_LEN as u32 );

            for tt in 0..FFT_LEN
            {
                s16_writer.write_sample( smp_buf[ tt ] );
            }

            if let Err( x ) = s16_writer.flush()
            {
                log::error!( "{:?}", x );
                return false;
            }
        }

        if !opt.skip_fft
        {
            let fft_result = fft.forward( &fft_buf );

            if opt.enable_fftbw
            {
                let fft_result = fft.backward( &fft_result );

                for tt in 0..FFT_LEN
                {
                    fft_buf_r[ tt ].re = fft_result[ tt ].re;
                    fft_buf_r[ tt ].im = fft_result[ tt ].im;
                }
            }

            c += 1;
        }

        if let Some( ref mut writer ) = writer_bw
        {
            let mut s16_writer = writer.get_i16_writer( FFT_LEN as u32 );

            for tt in 0..FFT_LEN
            {
                let s = fft_buf_r[ tt ].re;
                let s = s * t_i16_max;
                let s = s.to_i16().unwrap();

                s16_writer.write_sample( s );
            }

            if let Err( x ) = s16_writer.flush()
            {
                log::error!( "{:?}", x );
                return false;
            }
        }

        if t >= t_end
        {
            break;
        }

        t += FFT_LEN as usize;

        let ts = ( t - t % SAMPLE_RATE as usize ) / SAMPLE_RATE as usize;

        if ts != 0 && ts % 30 == 0 && ts > ls
        {
            ls = ts;
            log::info!( "{} sec done", ts );
        }
    }

    if let Some( writer ) = writer_bw
    {
        if let Err( x ) = writer.finalize()
        {
            log::error!( "{:?}", x );
            return false;
        }
    }

    if let Some( writer ) = writer
    {
        if let Err( x ) = writer.finalize()
        {
            log::error!( "{:?}", x );
            return false;
        }
    }

    log::info!( "fft count       [{}] ",    c );

    let tm_ed = Utc::now();

    let ts = t as f64 / SAMPLE_RATE as f64;
    let tm = ( tm_ed - tm_st ).num_milliseconds() as f64 / 1000.0;

    log::info!( "wav sample len  [{}]",     t );

    if opt.wav != ""
    {
        if let Ok( p ) = metadata( &opt.wav )
        {
            log::info!( "wav file   len  [{}]",      p.len() );
        }
    }

    log::info!( "wav time    (1) [{:9.4}] sec ",    ts );
    log::info!( "proc time   (2) [{:9.4}] sec ",    tm );
    log::info!( "(1) / (2)       [{:9.4}]     ",    ts / tm );

    true
}

fn main()
{
    if cfg!(debug_assertions)
    {
        if let Err(_) = env::var( "RUST_LOG" )
        {
            std::env::set_var( "RUST_LOG", "debug" );
        }
    }

    pretty_env_logger::init();

    let ( opt, freqs ) = parse_opt();

    if !(
        if opt.f64
        {
            let freqs = freqs.iter().map( | x | *x as f64 ).collect::< Vec< f64 > >();
            bench::< f64 >( opt, &freqs )
        }
        else
        {
            bench::< f32 >( opt, &freqs )
        }
    )
    {
        std::process::exit( 1 );
    }
}
