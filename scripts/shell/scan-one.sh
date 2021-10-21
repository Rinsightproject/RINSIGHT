selfpath=$(cd "$(dirname "$0")"; pwd) 
rust_proj_path=$(cd ""${selfpath}"/../.."; pwd)
output_path=$rust_proj_path"/data"

if [ $# -eq 0 ];
then 
    echo "please input a target folder to scan"
    exit 1
fi

skip="false"
debug_mode="false"

if [ $# -gt 1 ];
then
    for arg in ${@:1};do
        if [ $arg  == "--skip" ];
        then
            skip="--skip"
            echo $skip
        elif [ $arg  == "--debug" ] ;
        then
            debug_mode="--debug"
            echo $debug_mode
        fi
    done
fi

scan_root=$1

if [ ! -d ${scan_root} ];
then
    echo "scan root not exists or not a dir"
    exit 2
fi
category_name=`basename ${scan_root}`
echo "------------------------------${category_name}----------------------------------------------"
echo "-------------------------------------------------------------------------------------------"
cate_output_path="${output_path}/${category_name}"
if [ ! -d "${output_path}/${category_name}" ];
then
    echo "make dir ${cate_output_path}"
    mkdir "${cate_output_path}"
fi

for proj_path in ${scan_root}/* ;
do
    if test -d $proj_path ;
    then
        proj_name=`basename $proj_path`
        
        if [ $skip == "--skip" ] && [ -f "${cate_output_path}/${proj_name}" ];
        then
            echo "skip new scan for project ${proj_name}"
        else
            echo "new scan for project ${proj_name}"
            if [ $debug_mode == "--debug" ];
            then 
                cargo run "${scan_root}/${proj_name}" > "${cate_output_path}/${proj_name}"
            else
                cargo run --release "${scan_root}/${proj_name}" > "${cate_output_path}/${proj_name}"
            fi
        fi
        echo "--------------------------------"
    else
        echo "skip non-project file ${proj_name}"
    fi
done
if [ $debug_mode == "--debug" ];
then
    cargo run "${scan_root}" > "${cate_output_path}/cur_scan_total"
else
    cargo run --release "${scan_root}" > "${cate_output_path}/cur_scan_total"
fi





