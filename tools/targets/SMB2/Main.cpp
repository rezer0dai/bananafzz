#include <cstdio>
#include <iostream>

#include <SDL2/SDL.h>

#include "Emulation/Controller.hpp"
#include "SMB/SMBEngine.hpp"
#include "Util/Video.hpp"

#include "Configuration.hpp"
#include "Constants.hpp"

#include <stdio.h>

#include <thread>
#include <sys/shm.h>


//extern "C" void ijon_max(uint32_t a, uint64_t b);

uint8_t* romImage;
static SDL_Window* window;
static SDL_Renderer* renderer;
static SDL_Texture* texture;
static SDL_Texture* scanlineTexture;
static SMBEngine* smbEngine = nullptr;
static uint32_t renderBuffer[RENDER_WIDTH * RENDER_HEIGHT];


static int level = 0;

/**
 * Load the Super Mario Bros. ROM image.
 */
static bool loadRomImage()
{
    FILE* file = fopen(Configuration::getRomFileName().c_str(), "r");
    if (file == NULL)
    {
        std::cout << "Failed to open the file \"" << Configuration::getRomFileName() << "\". Exiting.\n";
        return false;
    }

    // Find the size of the file
    fseek(file, 0L, SEEK_END);
    size_t fileSize = ftell(file);
    fseek(file, 0L, SEEK_SET);

    // Read the entire file into a buffer
    romImage = new uint8_t[fileSize];
    fread(romImage, sizeof(uint8_t), fileSize, file);
    fclose(file);

    return true;
}

/**
 * SDL Audio callback function.
 */
static void audioCallback(void* userdata, uint8_t* buffer, int len)
{
    if (smbEngine != nullptr)
    {
        smbEngine->audioCallback(buffer, len);
    }
}

bool initialized = false;

/**
 * Initialize libraries for use.
 */
static bool initialize(bool video)
{
    if (initialized) {
        return true; 
    } initialized = true;

    // Load the configuration
    //
    Configuration::initialize(CONFIG_FILE_NAME);

    // Load the SMB ROM image
		if (!loadRomImage())
		{
				return false;
		}

		if(video){
			// Initialize SDL
			if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO) < 0)
			{
					std::cout << "SDL_Init() failed during initialize(): " << SDL_GetError() << std::endl;
					return false;
			}

			// Create the window
			window = SDL_CreateWindow(APP_TITLE,
																SDL_WINDOWPOS_UNDEFINED,
																SDL_WINDOWPOS_UNDEFINED,
																RENDER_WIDTH * Configuration::getRenderScale(),
																RENDER_HEIGHT * Configuration::getRenderScale(),
																0);
			if (window == nullptr)
			{
					std::cout << "SDL_CreateWindow() failed during initialize(): " << SDL_GetError() << std::endl;
					return false;
			}

			// Setup the renderer and texture buffer
			renderer = SDL_CreateRenderer(window, -1, (Configuration::getVsyncEnabled() ? SDL_RENDERER_PRESENTVSYNC : 0) | SDL_RENDERER_ACCELERATED);
			if (renderer == nullptr)
			{
					std::cout << "SDL_CreateRenderer() failed during initialize(): " << SDL_GetError() << std::endl;
					return false;
			}

			if (SDL_RenderSetLogicalSize(renderer, RENDER_WIDTH, RENDER_HEIGHT) < 0)
			{
					std::cout << "SDL_RenderSetLogicalSize() failed during initialize(): " << SDL_GetError() << std::endl;
					return false;
			}

			texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, RENDER_WIDTH, RENDER_HEIGHT);
			if (texture == nullptr)
			{
					std::cout << "SDL_CreateTexture() failed during initialize(): " << SDL_GetError() << std::endl;
					return false;
			}

			if (Configuration::getScanlinesEnabled())
			{
					scanlineTexture = generateScanlineTexture(renderer);
			}

			// Set up custom palette, if configured
			//
			if (!Configuration::getPaletteFileName().empty())
			{
					const uint32_t* palette = loadPalette(Configuration::getPaletteFileName());
					if (palette)
					{
							paletteRGB = palette;
					}
			}

			if (Configuration::getAudioEnabled())
			{
					// Initialize audio
					SDL_AudioSpec desiredSpec;
					desiredSpec.freq = Configuration::getAudioFrequency();
					desiredSpec.format = AUDIO_S8;
					desiredSpec.channels = 1;
					desiredSpec.samples = 2048;
					desiredSpec.callback = audioCallback;
					desiredSpec.userdata = NULL;

					SDL_AudioSpec obtainedSpec;
					SDL_OpenAudio(&desiredSpec, &obtainedSpec);

					// Start playing audio
					SDL_PauseAudio(0);
			}
		}

    return true;
}

/**
 * Shutdown libraries for exit.
 */
static void shutdown()
{
    SDL_CloseAudio();

    SDL_DestroyTexture(scanlineTexture);
    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);

    SDL_Quit();
}

uint8_t keys[6] = {0};

size_t relen(size_t len, size_t magnitude) {
    return len * magnitude;
}
uint8_t* input_re_alloc_impl(void* data, size_t len, size_t magnitude) {
    size_t new_len = relen(len, magnitude);
    uint8_t* new_data = (uint8_t*)malloc(new_len);
    if (!new_data)
        return 0;
    if (0 != data)
        memcpy(new_data, data, len);
    return new_data;
}
uint8_t* input_re_alloc(void* data, size_t len, size_t magnitude) {
    uint8_t* new_data = input_re_alloc_impl(data, len, magnitude);
    if (0 != data)
        free(data);
    return new_data;
}

void debug(char* logfile, char* msg) {
    FILE* file = fopen(logfile, "r+");
    fseek(file, 0L, SEEK_END);
    fprintf(file, msg);
    fclose(file);
}

#pragma pack push(1)                                                            
struct one_call {                                                               
    uint32_t offset;                                                              
    uint32_t size;                                                                
    uint32_t type;                                                                
};                                                                              
struct state_header {                                                           
    size_t magic;
    uint32_t splice_offset_to;                                                          
    uint32_t splice_offset_from;                                                          
    uint32_t poc_size;                                                          
    uint32_t total_count;                                                         
};                                                                              
#pragma pack pop(1)

bool play(int* pos) {
    if (0 == *pos) {
        return false;
    }
    (*pos)--;
    return true;
}


int banana(int com, bool fuzzing) {
    size_t len = 0x1000;
    size_t magnitude = 10;
    uint8_t* data = input_re_alloc(0, len, magnitude);
    if (!data)
        return -2;
    len = relen(len, magnitude);

//    size_t ind = read(STDIN_FILENO, data, len);
//    write(com, data, ind);

char* msg, *pos;
if (!fuzzing) {
    msg = (char*)malloc(1000);
    pos = msg + sprintf(msg, "SMB3-bug:1-1>");
}

	size_t ind = 0;
    while (read(STDIN_FILENO, data + ind++, 1) == 1) {

if (!fuzzing) pos += sprintf(pos, "%x+", data[ind-1]); 

        if (ind != len)
            continue;
        data = input_re_alloc(data, len, magnitude);
        if (!data)
            return -2;
        len = relen(len, magnitude);
    }

if (!fuzzing) {
    sprintf(pos, ":::(%x)", ind);
    debug("BUG_REPORT.txt", msg);
}

    write(com, data, len);
}

void bananafzz(SMBEngine& engine, int com) {
    banana(com, true);
    close(com);
}

bool finished = false;
bool running = true;

static void mainLoop(int com0, int com3, bool video, bool trace)
{

    int progStartTime = 0;
    if(video) { progStartTime = SDL_GetTicks(); }
    int frame = 0;
    int sleep = 100;
    int enter = 0;

    uint64_t last_world_pos = 0xffffffff;
    uint64_t idle = 0;
    bool hammertime = false;
    uint32_t pos_x = 0;

    size_t zzz = 0;
		//__AFL_INIT();

    while (running || video)
    {
//        printf("\n ** COMMUNICATION **\n");

        write(com3, &pos_x, 4);

        uint8_t ch;
        if(read(com0, &ch, 1) <1 ){
            break;
        } int right = ch;// % 8;
//        assert(right < 11);
        if(read(com0, &ch, 1) <1 ){
            break;
        } int left = ch;// % 8;
        if(read(com0, &ch, 1) <1 ){
            break;
        } int climb = ch;// % 8;
        if(read(com0, &ch, 1) <1 ){
            break;
        } int crowl = ch;// % 8;
        if(read(com0, &ch, 1) <1 ){
            break;
        } int jump = ch;// % 8;
        if(read(com0, &ch, 1) <1 ){
            break;
        } int fire = ch;// % 8;
        if(read(com0, &ch, 1) <1 ){
            break;
        } enter = ch;

        while ((running || video) && (
                right > 0 || 
                left > 0 || 
                climb > 0 || 
                crowl > 0 || 
                jump > 0 || 
                fire > 0) )
        {
//            smbEngine->writeData(0x0760, 5);

            if(video){
                SDL_Event event;
                while (SDL_PollEvent(&event))
                {
                        switch (event.type)
                        {
                        case SDL_QUIT:
                                running = false;
                                break;
                        case SDL_WINDOWEVENT:
                                switch (event.window.event)
                                {
                                case SDL_WINDOWEVENT_CLOSE:
                                        running = false;
                                        break;
                                }
                                break;

                        default:
                                break;
                        }
                }
            }
//            smbEngine->writeData(0x0760, 5);

            Controller& controller1 = smbEngine->getController1();


//            printf("(%i)", enter);

            controller1.setButtonState(BUTTON_START, 1);//enter-- > 0);//enter);
//            smbEngine->writeData(0x0760, 5);
            controller1.setButtonState(BUTTON_RIGHT, play(&right));//move
            controller1.setButtonState(BUTTON_LEFT, play(&left));//move
            controller1.setButtonState(BUTTON_UP, play(&climb));//climb
            controller1.setButtonState(BUTTON_DOWN, play(&crowl));//slide
            controller1.setButtonState(BUTTON_A, play(&jump));//jump
            controller1.setButtonState(BUTTON_B, play(&fire));//fire
            smbEngine->update();
            if (!video)
                smbEngine->render(renderBuffer);

//            enter = 0;

            if(video){

                const Uint8* sdl_keys = SDL_GetKeyboardState(NULL);
                if (sdl_keys[SDL_SCANCODE_ESCAPE])
                {
                        // quit
                        running = false;
                        break;
                }
                smbEngine->render(renderBuffer);

                SDL_UpdateTexture(texture, NULL, renderBuffer, sizeof(uint32_t) * RENDER_WIDTH);

                SDL_RenderClear(renderer);

                // Render the screen
                SDL_RenderSetLogicalSize(renderer, RENDER_WIDTH, RENDER_HEIGHT);
                SDL_RenderCopy(renderer, texture, NULL, NULL);

                // Render scanlines
                //
                if (Configuration::getScanlinesEnabled())
                {
                        SDL_RenderSetLogicalSize(renderer, RENDER_WIDTH * 3, RENDER_HEIGHT * 3);
                        SDL_RenderCopy(renderer, scanlineTexture, NULL, NULL);
                }

                SDL_RenderPresent(renderer);

        /**
         * Ensure that the framerate stays as close to the desired FPS as possible. If the frame was rendered faster, then delay. 
         * If the frame was slower, reset time so that the game doesn't try to "catch up", going super-speed.
         */
                int now = SDL_GetTicks();
                int delay = progStartTime + int(double(frame) * double(MS_PER_SEC) / double(Configuration::getFrameRate())) - now;
                if(delay > 0) 
                {
                        SDL_Delay(delay);
                }
                else 
                {
                        frame = 0;
                        progStartTime = now;
                }
            }

            uint64_t screen  = (uint64_t)smbEngine->readData(0x6d);
            uint64_t pos  = (uint64_t)smbEngine->readData(0x86);
            uint64_t world_pos = screen*255 + pos;

            pos_x = (uint32_t)world_pos;// / 3;



            if(smbEngine->readData(0x07A0) > 0){ //skip pre level timer
                smbEngine->writeData(0x07a0, 0);
            }
            if(smbEngine->readData(0x0e) == 0x0b){break;} //exit if dead
            if(smbEngine->readData(0xb5) > 0x01){break;} //exit if falling below screen
            if(world_pos > 44 && !hammertime){
                hammertime = true;
            }
            if(world_pos == last_world_pos){
                idle += 1;
            }else{ idle = 0; last_world_pos = world_pos; }
//				if(hammertime && idle > 4){break;} //lazy bastard
if (smbEngine->readData(0x1d) == 0x03) debug("CRASH.txt", "+");
            assert(smbEngine->readData(0x1d) != 0x03);
            frame++;


        }
    }
    finished=true;
    close(com0);
    close(com3);
}

bool video = false;//true;//
bool trace = false;

int com[4];
std::thread fzz;

extern "C"
void cLLVMFuzzerTestJoin() {
    running = false;
    close(com[1]);
    fzz.join();

    while (!finished)
        sleep(1);

    close(com[0]);
    close(com[2]);
    close(com[3]);
    finished = true;
    smbEngine->reset();
}

#pragma pack(push, 1)
struct pos {
    int8_t x;
    int8_t y;
};
struct bananafzz_out {
    uint32_t x;
    uint32_t y;
    uint8_t cash;
    uint8_t power;
    pos mario;
    pos coin;
    pos shroom;
    pos enemy[5];
};
#pragma pack(pop)

size_t cash = 0;
size_t power = 0;

size_t wp(size_t x) {
    uint64_t screen = smbEngine->readData(0x6d);
    return screen*255 + x;
}

extern "C"
void load_pos(uint8_t* pos) {
    bananafzz_out& banana = *(bananafzz_out*)pos;

    if (smbEngine->readData(0x0e) == 0x0b 
            || smbEngine->readData(0xb5) > 0x01) {
        //dead or bellow screen
        banana.x = 0;
        banana.mario.x = 0;
        banana.mario.y = 0;
        return;
    }

    uint64_t world_pos = wp(smbEngine->readData(0x86));

    //absolute positions of mario
    banana.x = world_pos;
    banana.y = smbEngine->readData(0xCE);

    banana.cash = smbEngine->readData(0x75E);

    banana.power = smbEngine->readData(0x756);

    //if only mario goes, then we want to go here
    banana.mario.x = 50;
    banana.mario.y = 0;

    //coin need to be seted internally by bananafzz
    banana.coin.x = 0;
    banana.coin.y = 0;

    size_t x = smbEngine->readData(0x86);
    size_t y = smbEngine->readData(0x3B8);

    //if shroom seen, then try to hit it!
    if (0x2e == smbEngine->readData(0x1B)) {
        banana.shroom.x = wp(smbEngine->readData(0x8c)) - 8 - world_pos; 
        banana.shroom.y = smbEngine->readData(0xD4) - 8 - y;
    } else {
        banana.shroom.x = 0;
        banana.shroom.y = 0;
    }

    for (size_t i = 0; i < 5; i++) {
        if (0 != 0xF & smbEngine->readData(0x87 + i)) {
            banana.enemy[i].x = wp(smbEngine->readData(0x87 + i)) - 8 - x;
            banana.enemy[i].y = smbEngine->readData(0xCF + i) - 8 - y;
//            printf("\n--> shgaring enemy (%i) poss [%i, %i] x [%i x %i] full (%i vs %i)", i, banana.enemy[i].x, banana.enemy[i].y, smbEngine->readData(0x87 + i), x, wp(smbEngine->readData(0x87 + i)), world_pos);
                
        } else {
            banana.enemy[i].x = 0;
            banana.enemy[i].y = 0;
        }
    }

//    printf(";; LOADING POS : %x %x", banana.x, banana.y);fflush(stdout);
}

extern "C"
void do_move(uint8_t* data, size_t size, uint8_t* pos) {
    assert(0 == size % 7);
    for ( size_t i = 0; i < size; i += 7 ) {
        if (!running)
            break;
//printf("+");fflush(stdout);
        if (-1 == write(com[1], data + i, 7))
            break;
//printf("*");fflush(stdout);
        if (-1 == read(com[2], pos, 4))
            break;
//printf("!");fflush(stdout);
    }
    load_pos(pos);
}

bool reset() {
    running = true;
    smbEngine->reset();

    for (size_t i = 1; 40 != smbEngine->readData(0x86); i ^= 1)
    {
        Controller& controller1 = smbEngine->getController1();
        controller1.setButtonState(BUTTON_START, i);

//        smbEngine->writeData(0x760, 11);//5//0

        smbEngine->update();
    }
    while (41 > smbEngine->readData(0x86))
    {
        Controller& controller1 = smbEngine->getController1();
        controller1.setButtonState(BUTTON_START, 1);
        controller1.setButtonState(BUTTON_RIGHT, 1);
        smbEngine->update();
    }
    return 1 == smbEngine->readData(0x770);//normal start
}

extern "C"
int cLLVMFuzzerTestOneInput(uint8_t* data, size_t size) {
    if (smbEngine) delete smbEngine;
    smbEngine = new SMBEngine(romImage);
    finished = false;
    if (!reset())
        return 1;

    if (-1 == pipe(&com[0]))
        return 0;
    if (-1 == pipe(&com[2]))
        return 0;

    fzz = std::thread([]() {
        mainLoop(com[0], com[3], video, trace);
        });

    uint8_t pos[10];
    if (-1 == read(com[2], pos, 4))
        return 0;

    if (!size) 
        return 0;

assert(false);

    do_move(data, size - (size % 7), pos);
    cLLVMFuzzerTestJoin();
    return 0;
}

extern "C"
int cLLVMFuzzerInitialize(int *, unsigned char***) {
		//ijon_map_set(0);
		printf("run level %d\n", level);


        level = 5;//21;//16;//0;//

		if(level >= 36 || level < 0){
			printf("ERROR: invalid level...\n");
			printf("===== Levels: =====\n");
			printf("0:      Level 1-1\n");
			printf("1:  Pre Level 1-2\n");
			printf("2:      Level 1-2\n");
			printf("3:      Level 1-3\n");
			printf("4:      Level 1-4\n");
			printf("5:      Level 2-1\n");
			printf("6:  Pre Level 2-2\n");
			printf("7:      Level 2-2\n");
			printf("8:      Level 2-3\n");
			printf("9:      Level 2-4\n");
			printf("10:     Level 3-1\n");
			printf("11:     Level 3-2\n");
			printf("12:     Level 3-3\n");
			printf("13:     Level 3-4\n");
			printf("14:     Level 4-1\n");
			printf("15: Pre Level 4-2\n");
			printf("16:     Level 4-2\n");
			printf("17:     Level 4-3\n");
			printf("18:     Level 4-4\n");
			printf("19:     Level 5-1\n");
			printf("20:     Level 5-2\n");
			printf("21:     Level 5-3\n");
			printf("22:     Level 5-4\n");
			printf("23:     Level 6-1\n");
			printf("24:     Level 6-2\n");
			printf("25:     Level 6-3\n");
			printf("26:     Level 6-4\n");
			printf("27:     Level 7-1\n");
			printf("28: Pre Level 7-2\n");
			printf("29:     Level 7-2\n");
			printf("30:     Level 7-3\n");
			printf("31:     Level 7-4\n");
			printf("32:     Level 8-1\n");
			printf("33:     Level 8-2\n");
			printf("34:     Level 8-3\n");
			printf("35:     Level 8-4\n");
		}

		

		printf("running mainLoop\n");

    if (!initialize(video))
    {
        std::cout << "Failed to initialize. Please check previous error messages for more information. The program will now exit.\n";
        return -1;
    }

    //smbEngine = new SMBEngine(romImage);

    return 0;
}

int xxmain() {
//    video = true;

    cLLVMFuzzerInitialize(0, 0);

    uint8_t data[0x1000];
    FILE* file = fopen("./d0607e4206a4c354", "r");

    fseek(file, 0L, SEEK_END);
    size_t fileSize = ftell(file);
    fseek(file, 0L, SEEK_SET);

    fread(data, sizeof(uint8_t), fileSize, file);
    fclose(file);

    for(size_t i = 0; i < 10; i++)
        cLLVMFuzzerTestOneInput(data, fileSize);

    if (video)
        shutdown();

    return 0;
}
extern "C"
int LLVMFuzzerInitialize(int *, unsigned char***) { return cLLVMFuzzerInitialize(0, 0); }
extern "C"
int LLVMFuzzerTestOneInput(uint8_t* _poc_mem, uint8_t* data, size_t size) {
    return cLLVMFuzzerTestOneInput(data, size);
}
